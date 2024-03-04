// Copyright 2022, Pulumi Corporation.  All rights reserved.

package executors

import (
	"fmt"

	"github.com/virtuslab/besom/language-host/fsys"
)

// WasmExecutor abstracts interactions with a wasm project, ability to build, run
// wasm code, and detect plugin dependencies.
type WasmExecutor struct {
	// User-friendly name of the executor.
	Name string

	// Path to the command to run.
	Cmd string

	// Optional dir to run the command in.
	Dir string

	// Command to run the Scala code - the main entrypoint.
	RunArgs []string

	// Command args to resolve dependencies and build; this will
	// be called after `pulumi new` on Gradle templates. Optional.
	BuildArgs []string

	// Command to autodetect and print Pulumi plugins depended on
	// by the Scala program.
	PluginArgs []string

	// Command to print the version of the command.
	VersionArgs []string
}

// WasmExecutorOptions contains information used to pick an executor.
type WasmExecutorOptions struct {
	// Current working directory. Abstract to enable testing.
	WD fsys.ParentFS

	// The absolute path to the bootstrap library jar
	// which should be shipped together with the main language host binary
	BootstrapLibJarPath string

	// The value of `runtime.options.binary` setting from
	// `Pulumi.yaml`. Optional.
	Binary string

	// The value of `runtime.options.use-executor` setting from
	// `Pulumi.yaml`. Optional.
	UseExecutor string
}

type wasmExecutorFactory interface {
	// NewWasmExecutor tries configuring an executor from the given options.
	// May return nil if options are not-applicable.
	NewWasmExecutor(WasmExecutorOptions) (*WasmExecutor, error)
}

func NewScalaExecutor(opts WasmExecutorOptions) (*WasmExecutor, error) {
	e, err := combineWasmExecutorFactories(
		&windows{},
	).NewWasmExecutor(opts)
	if err != nil {
		return nil, err
	}
	if e == nil {
		return nil, fmt.Errorf("failed to configure executor, tried: jar, sbt, scala-cli")
	}
	return e, nil
}

type combinedWasmExecutorFactory []wasmExecutorFactory

func (c combinedWasmExecutorFactory) NewWasmExecutor(opts WasmExecutorOptions) (*WasmExecutor, error) {
	for _, v := range c {
		e, err := v.NewWasmExecutor(opts)
		if err != nil {
			return nil, err
		}
		if e != nil {
			return e, nil
		}
	}
	return nil, nil
}

func combineWasmExecutorFactories(variations ...wasmExecutorFactory) wasmExecutorFactory {
	return combinedWasmExecutorFactory(variations)
}
