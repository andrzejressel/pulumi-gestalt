package main

import (
	"context"
	"fmt"
	"os"

	"github.com/pulumi/pulumi-go-provider/infer"
)

func main() {
	// We tell the provider what resources it needs to support.
	// In this case, a single custom resource called CombineString.
	p, err := infer.NewProviderBuilder().
		WithResources(
			infer.Resource(CombineString{}),
		).
		WithConfig(
			infer.Config(&Config{}),
		).
		Build()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	p.Run(context.Background(), "test", "0.1.0")
}

type Config struct {
	Prefix *string `pulumi:"prefix,optional"`
}

// Each resource has a controlling struct.
type CombineString struct{}

// Each resource has in input struct, defining what arguments it accepts.
type CombineStringArgs struct {
	Suffix string `pulumi:"suffix"`
}

// Each resource has a state, describing the fields that exist on the created resource.
type HelloWorldState struct {
	// It is generally a good idea to embed args in outputs, but it isn't strictly necessary.
	CombineStringArgs
	// Here we define a required output called message.
	Result string `pulumi:"result"`
}

// All resources must implement Create at a minimum.
func (CombineString) Create(
	ctx context.Context, req infer.CreateRequest[CombineStringArgs],
) (infer.CreateResponse[HelloWorldState], error) {
	config := infer.GetConfig[Config](ctx)
	prefixPtr := config.Prefix
	prefix := ""
	if prefixPtr != nil {
		prefix = *prefixPtr
	}
	name := req.Name
	inputs := req.Inputs
	state := HelloWorldState{CombineStringArgs: inputs}
	if req.DryRun {
		return infer.CreateResponse[HelloWorldState]{ID: name, Output: state}, nil
	}
	state.Result = prefix + inputs.Suffix
	return infer.CreateResponse[HelloWorldState]{ID: name, Output: state}, nil
}

func (r *CombineString) Annotate(a infer.Annotator) {
	a.Describe(&r, "Combines provider prefix with provided string")
}
