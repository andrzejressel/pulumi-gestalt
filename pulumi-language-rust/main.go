package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path"
	"path/filepath"
	"strings"
	"time"

	"github.com/andrzejressel/pulumi-gestalt/pulumi-language-rust/codegen/rust"
	"github.com/hashicorp/hcl/v2"
	hclsyntax "github.com/pulumi/pulumi/pkg/v3/codegen/hcl2/syntax"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"github.com/pulumi/pulumi/sdk/v3/go/common/encoding"
	"github.com/pulumi/pulumi/sdk/v3/go/common/resource/plugin"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/cmdutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/logging"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/rpcutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/workspace"
	pulumirpc "github.com/pulumi/pulumi/sdk/v3/proto/go"
	"go.opentelemetry.io/otel"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/emptypb"
)

const pluginVersion = "dev"

func main() {
	var tracing string
	flag.StringVar(&tracing, "tracing", "", "Emit tracing to a Zipkin-compatible tracing endpoint")
	flag.Parse()

	args := flag.Args()
	logging.InitLogging(false, 0, false)
	cmdutil.InitTracing("pulumi-language-rust", "pulumi-language-rust", tracing)

	var cancelChannel chan bool

	// Use OTel when the CLI provides an OTLP endpoint; fall back to
	// OpenTracing otherwise.  Only one system should be active to avoid
	// duplicate spans.
	otelEndpoint := os.Getenv("OTEL_EXPORTER_OTLP_ENDPOINT")
	if otelEndpoint == "" {
		cmdutil.InitTracing("pulumi-language-rust", "pulumi-language-rust", tracing)
	} else {
		if err := cmdutil.InitOtelTracing("pulumi-language-rust", otelEndpoint); err != nil {
			logging.V(3).Infof("failed to initialize OTel tracing: %v", err)
		}
		defer cmdutil.CloseOtelTracing()
	}

	// Optionally pluck out the engine so we can do logging, etc.
	var engineAddress string
	if len(args) > 0 {
		engineAddress = args[0]
		var err error
		cancelChannel, err = setupHealthChecks(engineAddress)
		if err != nil {
			cmdutil.Exit(fmt.Errorf("could not start health check host RPC server: %w", err))
		}
	}

	handle, err := rpcutil.ServeWithOptions(rpcutil.ServeOptions{
		Cancel: cancelChannel,
		Init: func(srv *grpc.Server) error {
			host := newLanguageHost(engineAddress, tracing, otelEndpoint, false)
			pulumirpc.RegisterLanguageRuntimeServer(srv, host)
			return nil
		},
		Options: rpcutil.OpenTracingServerInterceptorOptions(nil),
	})
	if err != nil {
		cmdutil.Exit(fmt.Errorf("could not start language host RPC server: %w", err))
	}

	fmt.Printf("%d\n", handle.Port)

	if err := <-handle.Done; err != nil {
		cmdutil.Exit(fmt.Errorf("language host RPC stopped serving: %w", err))
	}
}

func setupHealthChecks(engineAddress string) (chan bool, error) {
	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt)

	cancelChannel := make(chan bool)
	go func() {
		<-ctx.Done()
		cancel()
		close(cancelChannel)
	}()

	err := rpcutil.Healthcheck(ctx, engineAddress, 5*time.Minute, cancel)
	if err != nil {
		return nil, fmt.Errorf("could not start health checks: %w", err)
	}

	return cancelChannel, nil
}

type rustLanguageHost struct {
	pulumirpc.UnimplementedLanguageRuntimeServer
	engineAddress string
	tracing       string
	otelEndpoint  string
	testing       bool
}

func newLanguageHost(
	engineAddress, tracing, otelEndpoint string, testing bool,
) pulumirpc.LanguageRuntimeServer {
	return &rustLanguageHost{
		engineAddress: engineAddress,
		tracing:       tracing,
		otelEndpoint:  otelEndpoint,
		testing:       testing,
	}
}

func (host *rustLanguageHost) GetPluginInfo(context.Context, *emptypb.Empty) (*pulumirpc.PluginInfo, error) {
	return &pulumirpc.PluginInfo{Version: pluginVersion}, nil
}

func (host *rustLanguageHost) GetProgramDependencies(context.Context, *pulumirpc.GetProgramDependenciesRequest) (*pulumirpc.GetProgramDependenciesResponse, error) {
	return &pulumirpc.GetProgramDependenciesResponse{}, nil
}

func (host *rustLanguageHost) Run(_ context.Context, req *pulumirpc.RunRequest) (*pulumirpc.RunResponse, error) {

	directoryName := path.Base(req.Info.RootDirectory) + "-" + path.Base(req.Info.ProgramDirectory)

	config, err := host.constructConfig(req)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize configuration: %w", err)
	}
	configSecretKeys, err := host.constructConfigSecretKeys(req)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize configuration secret keys: %w", err)
	}

	var stdoutBuf bytes.Buffer
	var stderrBuf bytes.Buffer

	env := host.constructEnv(req, config, configSecretKeys)
	if host.testing {
		home, err := os.UserHomeDir()
		if err != nil {
			return nil, fmt.Errorf("could not get user home directory: %w", err)
		}
		env = append(env, fmt.Sprintf("CARGO_TARGET_DIR=%s/test_target/%s", home, directoryName))
	}
	env = append(env, "RUST_BACKTRACE=full")

	cmd := exec.Command("cargo", "run") // nolint: gosec
	cmd.Stdout = &stdoutBuf
	cmd.Stderr = &stderrBuf
	cmd.Dir = req.Info.ProgramDirectory
	cmd.Env = env

	var errResult string

	if err := runCommand(cmd); err != nil {
		os.Stdout.Write(stdoutBuf.Bytes())
		os.Stderr.Write(stderrBuf.Bytes())

		logging.V(5).Infof("InstallDependencies(Directory=%s): failed", req.Info.ProgramDirectory) //nolint:staticcheck
		return nil, fmt.Errorf("Failed to run command: %w", err)
	}

	return &pulumirpc.RunResponse{Error: errResult}, nil

}

// constructEnv constructs an environ for `pulumi-language-scala`
// by enumerating all the optional and non-optional evn vars present
// in a RunRequest.
func (host *rustLanguageHost) constructEnv(req *pulumirpc.RunRequest, config, configSecretKeys string) []string {
	env := os.Environ()

	maybeAppendEnv := func(k, v string) {
		if v != "" {
			env = append(env, strings.ToUpper("PULUMI_"+k)+"="+v)
		}
	}

	maybeAppendEnv("monitor", req.GetMonitorAddress())
	maybeAppendEnv("engine", host.engineAddress)
	maybeAppendEnv("project", req.GetProject())
	maybeAppendEnv("stack", req.GetStack())
	maybeAppendEnv("root_directory", req.GetInfo().GetRootDirectory())
	maybeAppendEnv("pwd", req.GetPwd())
	maybeAppendEnv("dry_run", fmt.Sprintf("%v", req.GetDryRun()))
	maybeAppendEnv("query_mode", fmt.Sprint(req.GetQueryMode()))
	maybeAppendEnv("parallel", fmt.Sprint(req.GetParallel()))
	maybeAppendEnv("tracing", host.tracing)
	maybeAppendEnv("config", config)
	maybeAppendEnv("config_secret_keys", configSecretKeys)
	maybeAppendEnv("organization", req.GetOrganization())

	return env
}

// constructConfig json-serializes the configuration data given as part of a RunRequest.
func (host *rustLanguageHost) constructConfig(req *pulumirpc.RunRequest) (string, error) {
	configMap := req.GetConfig()
	if configMap == nil {
		return "", nil
	}

	configJSON, err := json.Marshal(configMap)
	if err != nil {
		return "", fmt.Errorf("failed to serialize configuration: %w", err)
	}

	return string(configJSON), nil
}

// constructConfigSecretKeys JSON-serializes the list of keys that contain secret values given as part of
// a RunRequest.
func (host *rustLanguageHost) constructConfigSecretKeys(req *pulumirpc.RunRequest) (string, error) {
	configSecretKeys := req.GetConfigSecretKeys()
	if configSecretKeys == nil {
		return "[]", nil
	}

	configSecretKeysJSON, err := json.Marshal(configSecretKeys)
	if err != nil {
		return "", fmt.Errorf("failed to serialize configuration secret keys: %w", err)
	}

	return string(configSecretKeysJSON), nil
}

func (host *rustLanguageHost) InstallDependencies(
	req *pulumirpc.InstallDependenciesRequest,
	server pulumirpc.LanguageRuntime_InstallDependenciesServer,
) error {

	tracer := otel.Tracer("pulumi-language-rust")
	_, otelSpan := cmdutil.StartSpan(server.Context(), tracer, "rust-install-deps")
	defer otelSpan.End()

	closer, stdout, stderr, err := rpcutil.MakeInstallDependenciesStreams(server, req.IsTerminal)
	if err != nil {
		return fmt.Errorf("failed to create install dependency streams: %w", err)
	}
	defer closer.Close()

	directoryName := path.Base(req.Info.ProgramDirectory)

	env := os.Environ()
	if host.testing {
		home, err := os.UserHomeDir()
		if err != nil {
			return fmt.Errorf("could not get user home directory: %w", err)
		}
		env = append(env, fmt.Sprintf("CARGO_TARGET_DIR=%s/test_target/%s", home, directoryName))
	}

	cmd := exec.Command("cargo", "build") // nolint: gosec
	cmd.Dir = req.Info.ProgramDirectory
	cmd.Stdout = stdout
	cmd.Stderr = stderr
	cmd.Env = env

	if err := runCommand(cmd); err != nil {
		logging.V(5).Infof("InstallDependencies(Directory=%s): failed", req.Info.ProgramDirectory) //nolint:staticcheck
		return fmt.Errorf("cargo build failed: %w", err)
	}

	defer closer.Close()

	return nil
}

func (host *rustLanguageHost) GetRequiredPackages(
	context.Context,
	*pulumirpc.GetRequiredPackagesRequest,
) (*pulumirpc.GetRequiredPackagesResponse, error) {
	return &pulumirpc.GetRequiredPackagesResponse{Packages: []*pulumirpc.PackageDependency{}}, nil
}

func (host *rustLanguageHost) GenerateProgram(
	_ context.Context,
	req *pulumirpc.GenerateProgramRequest,
) (*pulumirpc.GenerateProgramResponse, error) {
	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, fmt.Errorf("failed to create loader client: %w", err)
	}
	defer loader.Close()
	files, diags, err := generateProgramFromSource(req.Source, schema.NewCachedLoader(loader), req.Strict)
	if err != nil {
		return nil, fmt.Errorf("failed to generate program from source: %w", err)
	}

	return &pulumirpc.GenerateProgramResponse{
		Source:      files,
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

func (host *rustLanguageHost) GenerateProject(_ context.Context, req *pulumirpc.GenerateProjectRequest) (*pulumirpc.GenerateProjectResponse, error) {
	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, fmt.Errorf("failed to create loader client: %w", err)
	}

	var extraOptions []pcl.BindOption
	if !req.Strict {
		extraOptions = append(extraOptions, pcl.NonStrictBindOptions()...)
	}

	extraOptions = append(extraOptions, pcl.PreferOutputVersionedInvokes)

	program, diags, err := pcl.BindDirectory(req.SourceDirectory, loader, extraOptions...)
	if err != nil {
		return nil, fmt.Errorf("failed to bind directory: %w", err)
	}

	if diags.HasErrors() {
		rpcDiagnostics := plugin.HclDiagnosticsToRPCDiagnostics(diags)
		return &pulumirpc.GenerateProjectResponse{
			Diagnostics: rpcDiagnostics,
		}, nil
	}

	if program == nil {
		return nil, errors.New("internal error: program was nil")
	}

	var project workspace.Project
	if err := json.Unmarshal([]byte(req.Project), &project); err != nil {
		return nil, fmt.Errorf("failed to deserialize project: %w", err)
	}

	err = generateProject(req.TargetDirectory, project, program, host.testing)
	if err != nil {
		return nil, fmt.Errorf("could not generate project: %w", err)
	}

	return &pulumirpc.GenerateProjectResponse{}, nil

}
func (host *rustLanguageHost) GeneratePackage(_ context.Context, req *pulumirpc.GeneratePackageRequest) (*pulumirpc.GeneratePackageResponse, error) {

	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, fmt.Errorf("failed to create loader client: %w", err)
	}

	var spec schema.PackageSpec
	err = json.Unmarshal([]byte(req.Schema), &spec)
	if err != nil {
		return nil, fmt.Errorf("failed to deserialize schema: %w", err)
	}

	diags := hcl.Diagnostics{}
	pkg, bindDiags, err := schema.BindSpec(spec, loader, schema.ValidationOptions{
		AllowDanglingReferences: true,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to bind schema spec: %w", err)
	}
	diags = diags.Extend(bindDiags)
	if bindDiags.HasErrors() {
		return &pulumirpc.GeneratePackageResponse{
			Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
		}, nil
	}

	err = rust.GeneratePackage(pkg, req.Directory)
	if err != nil {
		return nil, fmt.Errorf("failed to generate package: %w", err)
	}

	return &pulumirpc.GeneratePackageResponse{
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

// Pack Cargo does not have binary deployable packages
func (host *rustLanguageHost) Pack(ctx context.Context, request *pulumirpc.PackRequest) (*pulumirpc.PackResponse, error) {
	return &pulumirpc.PackResponse{}, nil
}

func generateProgramFromSource(
	source map[string]string,
	loader schema.ReferenceLoader,
	strict bool,
) (map[string][]byte, hcl.Diagnostics, error) {
	parser := hclsyntax.NewParser()
	for path, contents := range source {
		err := parser.ParseFile(strings.NewReader(contents), path)
		if err != nil {
			return nil, nil, fmt.Errorf("failed to parse source file %q: %w", path, err)
		}
	}
	if parser.Diagnostics.HasErrors() {
		return nil, parser.Diagnostics, nil
	}

	bindOptions := []pcl.BindOption{
		pcl.Loader(loader),
		pcl.PreferOutputVersionedInvokes,
	}
	if !strict {
		bindOptions = append(bindOptions, pcl.NonStrictBindOptions()...)
	}

	program, diags, err := pcl.BindProgram(parser.Files, bindOptions...)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to bind program: %w", err)
	}
	if diags.HasErrors() {
		return nil, diags, nil
	}
	if program == nil {
		return nil, nil, fmt.Errorf("internal error: program was nil")
	}

	files, generationDiags, err := rust.GenerateProgram(program)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to generate program: %w", err)
	}

	return files, diags.Extend(generationDiags), nil
}

func generateProject(
	directory string,
	project workspace.Project,
	program *pcl.Program,
	testing bool,
) error {
	rootDirectory := directory
	projectDirectory := directory
	if project.Main != "" {
		projectDirectory = filepath.Join(directory, project.Main)
	}

	_, protobufJSON, err := rust.GenerateProject(program, projectDirectory)
	if err != nil {
		return fmt.Errorf("failed to generate project files: %w", err)
	}

	// Set the runtime to "java" then marshal to Pulumi.yaml
	project.Runtime = workspace.NewProjectRuntimeInfo("rust", nil)
	projectBytes, err := encoding.YAML.Marshal(project)
	if err != nil {
		return fmt.Errorf("failed to marshal project to YAML: %w", err)
	}

	filesWithPackages := make(map[string][]byte)

	filesWithPackages[filepath.Join(rootDirectory, "Pulumi.yaml")] = projectBytes
	if testing {
		filesWithPackages[filepath.Join(rootDirectory, "protobuf.json")] = protobufJSON
	}
	//filesWithPackages[filepath.Join(rootDirectory, "Cargo.toml")] = []byte("[package]\nname=\"TEST\"")

	for filePath, data := range filesWithPackages {
		dir := filepath.Dir(filePath)
		err := os.MkdirAll(dir, os.ModePerm)
		if err != nil {
			return fmt.Errorf("could not create output directory %s: %w", dir, err)
		}
		err = os.WriteFile(filePath, data, 0o600)
		if err != nil {
			return fmt.Errorf("could not write output program: %w", err)
		}
	}

	return nil

}
