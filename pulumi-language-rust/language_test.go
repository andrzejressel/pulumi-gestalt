package main

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sync"
	"testing"

	"github.com/pulumi/pulumi/sdk/v3/go/common/util/contract"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/rpcutil"
	pulumirpc "github.com/pulumi/pulumi/sdk/v3/proto/go"
	testingrpc "github.com/pulumi/pulumi/sdk/v3/proto/go/testing"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// TestLanguage runs the language conformance test suite against the PCL AST language host.
func TestLanguage(t *testing.T) {

	if runtime.GOOS == "windows" {
		panic("Windows not yet supported")
	}

	t.Parallel()

	engineAddress, engine := runTestingHost(t)

	tests, err := engine.GetLanguageTests(context.Background(), &testingrpc.GetLanguageTestsRequest{})
	require.NoError(t, err)

	cancel := make(chan bool)

	var rootDir string
	if os.Getenv("LOCAL_TEST") == "1" {
		rootDir, err = filepath.Abs("/tmp/pulumi-tests")
		require.NoError(t, err)
	} else {
		rootDir = t.TempDir()
	}

	handle, err := rpcutil.ServeWithOptions(rpcutil.ServeOptions{
		Init: func(srv *grpc.Server) error {
			host := newLanguageHost(
				engineAddress,
				"", /*tracing*/
				"", /*otel*/
				true,
			)
			pulumirpc.RegisterLanguageRuntimeServer(srv, host)
			return nil
		},
		Cancel: cancel,
	})
	require.NoError(t, err)

	snapshotDir := "./testdata/"

	prepare, err := engine.PrepareLanguageTests(context.Background(), &testingrpc.PrepareLanguageTestsRequest{
		LanguagePluginName:   "rust",
		LanguagePluginTarget: fmt.Sprintf("127.0.0.1:%d", handle.Port),
		TemporaryDirectory:   rootDir,
		SnapshotDirectory:    snapshotDir,
		//CoreSdkVersion:       sdk.Version.String(), // It default to pulumi version - I would have to research it
		LanguageInfo: "{}",
		SnapshotEdits: []*testingrpc.PrepareLanguageTestsRequest_Replacement{
			{
				Path:        "Cargo.toml",
				Pattern:     `(?m)# PULUMI_GESTALT_RUST START(.|\n)*PULUMI_GESTALT_RUST END`,
				Replacement: "pulumi_gestalt_rust = \"VERSION\"",
			},
		},
	})
	require.NoError(t, err)

	for _, tt := range tests.Tests {
		tt := tt
		t.Run(tt, func(t *testing.T) {
			t.Parallel()

			if expected, ok := expectedFailures[tt]; ok {
				t.Skipf("Skipping known failure: %s", expected)
			}

			err := os.MkdirAll(filepath.Join(rootDir, "testdata", tt), os.ModePerm)
			require.NoError(t, err)

			result, err := engine.RunLanguageTest(context.Background(), &testingrpc.RunLanguageTestRequest{
				Token: prepare.Token,
				Test:  tt,
			})

			require.NoError(t, err)
			for _, msg := range result.Messages {
				t.Log(msg)
			}
			t.Logf("stdout: %s", result.Stdout)
			t.Logf("stderr: %s", result.Stderr)
			assert.True(t, result.Success)
		})
	}

	t.Cleanup(func() {
		close(cancel)
		assert.NoError(t, <-handle.Done)
	})
}

// expectedFailures maps the set of conformance tests we expect to fail to reasons they currently do so, so that we may
// skip them with an informative message until they are fixed.
var expectedFailures = map[string]string{
	"l1-builtin-can":                               "unsupported config variable type: map(string)",
	"l1-builtin-require-pulumi-version":            "unsupported node type: dynamic",
	"l1-builtin-secret":                            "unsupported config variable type: output(string)",
	"l1-builtin-stash":                             "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-27.0.0: no such file or directory",
	"l1-builtin-to-json":                           "unsupported config variable type: list(string)",
	"l1-builtin-try":                               "sdk snapshot validation for any-type-function: walk expected dir: lstat testdata/sdks/any-type-function-15.0.0: no such file or directory",
	"l1-config-secret":                             "unsupported config variable type: output(number)",
	"l1-config-types-object":                       "unsupported config variable type: map(int)",
	"l1-config-types-primitive":                    "unsupported operation type in expression lowering",
	"l1-elide-index":                               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l1-keyword-overlap":                           "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l1-output-map":                                "sdk snapshot validation for sync: walk expected dir: lstat testdata/sdks/sync-3.0.0-alpha.1.internal+exp.sha.2143768: no such file or directory",
	"l1-output-null":                               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l1-output-number":                             "program snapshot validation: walk expected dir: lstat testdata/projects/l1-output-number: no such file or directory",
	"l1-output-string":                             "sdk snapshot validation for asset-archive: walk expected dir: lstat testdata/sdks/asset-archive-5.0.0: no such file or directory",
	"l1-proxy-index":                               "unsupported config variable type: object with union members",
	"l1-stack-reference":                           "sdk snapshot validation for discriminated-union: walk expected dir: lstat testdata/sdks/discriminated-union-31.0.0: no such file or directory",
	"l2-builtin-object":                            "sdk snapshot validation for output: walk expected dir: lstat testdata/sdks/output-23.0.0: no such file or directory",
	"l2-camel-names":                               "sdk snapshot validation for camelNames: walk expected dir: lstat testdata/sdks/camelNames-19.0.0: no such file or directory",
	"l2-component-call-simple":                     "sdk snapshot validation for component: walk expected dir: lstat testdata/sdks/component-13.3.7: no such file or directory",
	"l2-component-component-resource-ref":          "sdk snapshot validation for component: walk expected dir: lstat testdata/sdks/component-13.3.7: no such file or directory",
	"l2-component-program-resource-ref":            "sdk snapshot validation for component: walk expected dir: lstat testdata/sdks/component-13.3.7: no such file or directory",
	"l2-component-property-deps":                   "sdk snapshot validation for component-property-deps: walk expected dir: lstat testdata/sdks/component-property-deps-1.33.7: no such file or directory",
	"l2-destroy":                                   "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-discriminated-union":                       "sdk snapshot validation for discriminated-union: walk expected dir: lstat testdata/sdks/discriminated-union-31.0.0: no such file or directory",
	"l2-elide-index":                               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-engine-update-options":                     "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-enum":                                      "sdk snapshot validation for enum: walk expected dir: lstat testdata/sdks/enum-30.0.0: no such file or directory",
	"l2-explicit-parameterized-provider":           "sdk snapshot validation for goodbye: walk expected dir: lstat testdata/sdks/goodbye-2.0.0: no such file or directory",
	"l2-explicit-provider":                         "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-explicit-providers":                        "sdk snapshot validation for component: walk expected dir: lstat testdata/sdks/component-13.3.7: no such file or directory",
	"l2-external-enum":                             "sdk snapshot validation for enum: walk expected dir: lstat testdata/sdks/enum-30.0.0: no such file or directory",
	"l2-failed-create":                             "sdk snapshot validation for fail_on_create: walk expected dir: lstat testdata/sdks/fail_on_create-4.0.0: no such file or directory",
	"l2-failed-create-continue-on-error":           "sdk snapshot validation for fail_on_create: walk expected dir: lstat testdata/sdks/fail_on_create-4.0.0: no such file or directory",
	"l2-invoke-dependencies":                       "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-invoke-options":                            "sdk snapshot validation for simple-invoke: walk expected dir: lstat testdata/sdks/simple-invoke-10.0.0: no such file or directory",
	"l2-invoke-options-depends-on":                 "sdk snapshot validation for simple-invoke: walk expected dir: lstat testdata/sdks/simple-invoke-10.0.0: no such file or directory",
	"l2-invoke-output-only":                        "sdk snapshot validation for output-only-invoke: walk expected dir: lstat testdata/sdks/output-only-invoke-24.0.0: no such file or directory",
	"l2-invoke-scalar":                             "sdk snapshot validation for simple-invoke-with-scalar-return: walk expected dir: lstat testdata/sdks/simple-invoke-with-scalar-return-17.0.0: no such file or directory",
	"l2-invoke-scalars":                            "sdk snapshot validation for scalar-returns: walk expected dir: lstat testdata/sdks/scalar-returns-21.0.0: no such file or directory",
	"l2-invoke-secrets":                            "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-invoke-simple":                             "sdk snapshot validation for simple-invoke: walk expected dir: lstat testdata/sdks/simple-invoke-10.0.0: no such file or directory",
	"l2-invoke-variants":                           "sdk snapshot validation for simple-invoke: walk expected dir: lstat testdata/sdks/simple-invoke-10.0.0: no such file or directory",
	"l2-keywords":                                  "sdk snapshot validation for keywords: walk expected dir: lstat testdata/sdks/keywords-20.0.0: no such file or directory",
	"l2-large-string":                              "sdk snapshot validation for large: walk expected dir: lstat testdata/sdks/large-4.3.2: no such file or directory",
	"l2-logical-name":                              "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-map-keys":                                  "sdk snapshot validation for plain: walk expected dir: lstat testdata/sdks/plain-13.0.0: no such file or directory",
	"l2-module-format":                             "sdk snapshot validation for module-format: walk expected dir: lstat testdata/sdks/module-format-29.0.0: no such file or directory",
	"l2-namespaced-provider":                       "sdk snapshot validation for component: walk expected dir: lstat testdata/sdks/component-13.3.7: no such file or directory",
	"l2-parallel-resources":                        "sdk snapshot validation for sync: walk expected dir: lstat testdata/sdks/sync-3.0.0-alpha.1.internal+exp.sha.2143768: no such file or directory",
	"l2-parameterized-invoke":                      "sdk snapshot validation for subpackage: walk expected dir: lstat testdata/sdks/subpackage-2.0.0: no such file or directory",
	"l2-parameterized-resource":                    "sdk snapshot validation for subpackage: walk expected dir: lstat testdata/sdks/subpackage-2.0.0: no such file or directory",
	"l2-parameterized-resource-twice":              "sdk snapshot validation for byepackage: walk expected dir: lstat testdata/sdks/byepackage-2.0.0: no such file or directory",
	"l2-plain":                                     "sdk snapshot validation for plain: walk expected dir: lstat testdata/sdks/plain-13.0.0: no such file or directory",
	"l2-primitive-ref":                             "sdk snapshot validation for primitive-ref: walk expected dir: lstat testdata/sdks/primitive-ref-11.0.0: no such file or directory",
	"l2-provider-call":                             "sdk snapshot validation for call: walk expected dir: lstat testdata/sdks/call-15.7.9: no such file or directory",
	"l2-provider-call-explicit":                    "sdk snapshot validation for call: walk expected dir: lstat testdata/sdks/call-15.7.9: no such file or directory",
	"l2-provider-grpc-config":                      "sdk snapshot validation for config-grpc: walk expected dir: lstat testdata/sdks/config-grpc-1.0.0: no such file or directory",
	"l2-provider-grpc-config-schema-secret":        "sdk snapshot validation for config-grpc: walk expected dir: lstat testdata/sdks/config-grpc-1.0.0: no such file or directory",
	"l2-provider-grpc-config-secret":               "sdk snapshot validation for config-grpc: walk expected dir: lstat testdata/sdks/config-grpc-1.0.0: no such file or directory",
	"l2-proxy-index":                               "sdk snapshot validation for ref-ref: walk expected dir: lstat testdata/sdks/ref-ref-12.0.0: no such file or directory",
	"l2-ref-ref":                                   "sdk snapshot validation for ref-ref: walk expected dir: lstat testdata/sdks/ref-ref-12.0.0: no such file or directory",
	"l2-resource-alpha":                            "sdk snapshot validation for alpha: walk expected dir: lstat testdata/sdks/alpha-3.0.0-alpha.1.internal+exp.sha.12345678: no such file or directory",
	"l2-resource-asset-archive":                    "sdk snapshot validation for asset-archive: walk expected dir: lstat testdata/sdks/asset-archive-5.0.0: no such file or directory",
	"l2-resource-config":                           "sdk snapshot validation for config: walk expected dir: lstat testdata/sdks/config-9.0.0: no such file or directory",
	"l2-resource-config-objects":                   "not yet implemented",
	"l2-resource-config-primitives":                "not yet implemented",
	"l2-resource-elide-unknowns":                   "sdk snapshot validation for output: walk expected dir: lstat testdata/sdks/output-23.0.0: no such file or directory",
	"l2-resource-invoke-dynamic-function":          "sdk snapshot validation for any-type-function: walk expected dir: lstat testdata/sdks/any-type-function-15.0.0: no such file or directory",
	"l2-resource-keyword-overlap":                  "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-name-type":                        "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-names":                            "sdk snapshot validation for names: walk expected dir: lstat testdata/sdks/names-6.0.0: no such file or directory",
	"l2-resource-option-additional-secret-outputs": "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-alias":                     "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-custom-timeouts":           "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-delete-before-replace":     "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-deleted-with":              "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-depends-on":                "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-env-var-mappings":          "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-hide-diffs":                "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-ignore-changes":            "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-import":                    "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-plugin-download-url":       "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-27.0.0: no such file or directory",
	"l2-resource-option-protect":                   "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-replace-on-changes":        "sdk snapshot validation for replaceonchanges: walk expected dir: lstat testdata/sdks/replaceonchanges-25.0.0: no such file or directory",
	"l2-resource-option-replace-with":              "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-replacement-trigger":       "sdk snapshot validation for output: walk expected dir: lstat testdata/sdks/output-23.0.0: no such file or directory",
	"l2-resource-option-retain-on-delete":          "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-option-version":                   "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-26.0.0: no such file or directory",
	"l2-resource-option-version-sdk":               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-optional":                         "sdk snapshot validation for optionalprimitive: walk expected dir: lstat testdata/sdks/optionalprimitive-34.0.0: no such file or directory",
	"l2-resource-order":                            "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-parent-inheritance":               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-resource-primitive-defaults":               "sdk snapshot validation for primitive-defaults: walk expected dir: lstat testdata/sdks/primitive-defaults-8.0.0: no such file or directory",
	"l2-resource-primitives":                       "sdk snapshot validation for primitive: walk expected dir: lstat testdata/sdks/primitive-7.0.0: no such file or directory",
	"l2-resource-provider-inheritance":             "sdk snapshot validation for primitive: walk expected dir: lstat testdata/sdks/primitive-7.0.0: no such file or directory",
	"l2-resource-secret":                           "sdk snapshot validation for secret: walk expected dir: lstat testdata/sdks/secret-14.0.0: no such file or directory",
	"l2-resource-simple":                           "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-snake-names":                               "sdk snapshot validation for snake_names: walk expected dir: lstat testdata/sdks/snake_names-33.0.0: no such file or directory",
	"l2-target-up-with-new-dependency":             "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"l2-union":                                     "sdk snapshot validation for union: walk expected dir: lstat testdata/sdks/union-18.0.0: no such file or directory",
	"l3-component-config-objects":                  "sdk snapshot validation for primitive: walk expected dir: lstat testdata/sdks/primitive-7.0.0: no such file or directory",
	"l3-component-config-primitives":               "sdk snapshot validation for primitive: walk expected dir: lstat testdata/sdks/primitive-7.0.0: no such file or directory",
	"l3-component-simple":                          "unsupported node type: object with output fields",
	"l3-deferred-outputs":                          "unsupported node type: object with output fields",
	"l3-for":                                       "unsupported config variable type: list(string)",
	"l3-for-resource":                              "unsupported expression type: for-expression",
	"l3-range":                                     "unsupported config variable type: list(string)",
	"l3-range-parent-scope":                        "sdk snapshot validation for nestedobject: walk expected dir: lstat testdata/sdks/nestedobject-1.42.0: no such file or directory",
	"l3-range-resource-output-traversal":           "sdk snapshot validation for nestedobject: walk expected dir: lstat testdata/sdks/nestedobject-1.42.0: no such file or directory",
	"l3-rewrite-conversions":                       "not yet implemented",
	"l3-splat":                                     "unsupported expression type: splat-expression",
	"policy-config":                                "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-config-schema":                         "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-dryrun":                                "unsupported operation type in expression lowering",
	"policy-enforcement-config":                    "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-invalid":                               "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-remediate":                             "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-simple":                                "unsupported operation type in expression lowering",
	"policy-stack-config":                          "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"policy-stack-tags":                            "sdk snapshot validation for simple: walk expected dir: lstat testdata/sdks/simple-2.0.0: no such file or directory",
	"provider-alias-component":                     "requires language providers directory support for conformance-component",
	"provider-ignore-changes-component":            "requires language providers directory support for conformance-component",
	"provider-replacement-trigger-component":       "requires language providers directory support for conformance-component",
	"provider-resource-component":                  "requires language providers directory support for conformance-component",
}

// runTestingHost boots up a new instance of the language conformance test runner, `pulumi-test-language`, as well as a
// fake Pulumi engine for collecting logs. It returns the address of the fake engine and a connection to the test runner
// that can be used to manage a test suite run.
func runTestingHost(t *testing.T) (string, testingrpc.LanguageTestClient) {
	// We can't just go run the pulumi-test-language package because of
	// https://github.com/golang/go/issues/39172, so we build it to a temp file then run that.
	binary := t.TempDir() + "/pulumi-test-language"
	cmd := exec.Command("go", "build", "-o", binary, "github.com/pulumi/pulumi/pkg/v3/testing/pulumi-test-language") //nolint:gosec,lll
	output, err := cmd.CombinedOutput()
	t.Logf("build output: %s", output)
	require.NoError(t, err)

	cmd = exec.Command(binary)
	stdout, err := cmd.StdoutPipe()
	require.NoError(t, err)
	stderr, err := cmd.StderrPipe()
	require.NoError(t, err)
	stderrReader := bufio.NewReader(stderr)

	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		for {
			text, err := stderrReader.ReadString('\n')
			if err != nil {
				wg.Done()
				return
			}
			t.Logf("engine: %s", text)
		}
	}()

	err = cmd.Start()
	require.NoError(t, err)

	stdoutBytes, err := io.ReadAll(stdout)
	require.NoError(t, err)

	address := string(stdoutBytes)

	conn, err := grpc.NewClient(
		address,
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithUnaryInterceptor(rpcutil.OpenTracingClientInterceptor()),
		grpc.WithStreamInterceptor(rpcutil.OpenTracingStreamClientInterceptor()),
		rpcutil.GrpcChannelOptions(),
	)
	require.NoError(t, err)

	client := testingrpc.NewLanguageTestClient(conn)

	t.Cleanup(func() {
		assert.NoError(t, cmd.Process.Kill())
		wg.Wait()
		// We expect this to error because we just killed it.
		contract.IgnoreError(cmd.Wait())
	})

	return address, client
}
