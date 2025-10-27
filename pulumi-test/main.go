package main

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/pulumi/pulumi-go-provider/infer"
)

func main() {
	// We tell the provider what resources it needs to support.
	// In this case, a single custom resource called HelloWorld.
	p, err := infer.NewProviderBuilder().
		WithResources(
			infer.Resource(HelloWorld{}),
		).
		Build()
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
    p.Run(context.Background(), "test", "0.1.0")
}

// Each resource has a controlling struct.
type HelloWorld struct{}

// Each resource has in input struct, defining what arguments it accepts.
type HelloWorldArgs struct {
	// Fields projected into Pulumi must be public and hava a `pulumi:"..."` tag.
	// The pulumi tag doesn't need to match the field name, but its generally a
	// good idea.
	Name string `pulumi:"name"`
	// Fields marked `optional` are optional, so they should have a pointer
	// ahead of their type.
	Loud *bool `pulumi:"loud,optional"`
}

// Each resource has a state, describing the fields that exist on the created resource.
type HelloWorldState struct {
	// It is generally a good idea to embed args in outputs, but it isn't strictly necessary.
	HelloWorldArgs
	// Here we define a required output called message.
	Message string `pulumi:"message"`
}

// All resources must implement Create at a minumum.
func (HelloWorld) Create(
	ctx context.Context, req infer.CreateRequest[HelloWorldArgs],
) (infer.CreateResponse[HelloWorldState], error) {
	name := req.Name
	inputs := req.Inputs
	state := HelloWorldState{HelloWorldArgs: inputs}
	if req.DryRun {
		return infer.CreateResponse[HelloWorldState]{ID: name, Output: state}, nil
	}
	state.Message = fmt.Sprintf("Hello, %s", inputs.Name)
	if inputs.Loud != nil && *inputs.Loud {
		state.Message = strings.ToUpper(state.Message)
	}
	return infer.CreateResponse[HelloWorldState]{ID: name, Output: state}, nil
}

func (r *HelloWorld) Annotate(a infer.Annotator) {
	a.Describe(&r, "Produces a Hello message.")
}