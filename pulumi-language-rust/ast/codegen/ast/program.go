package ast

import (
	"encoding/base64"
	"fmt"

	"github.com/andrzejressel/pulumi-gestalt/pulumi-language-rust/ast/codegen/shared"
	astproto "github.com/andrzejressel/pulumi-gestalt/pulumi-language-rust/ast/protobuf/schemapcl"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclsyntax"
	"github.com/pulumi/pulumi/pkg/v3/codegen/hcl2/model"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"github.com/zclconf/go-cty/cty"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

func transformProgramType(t model.Type) (*astproto.Type, error) {
	if t == nil {
		return nil, fmt.Errorf("type is nil")
	}

	switch t := t.(type) {
	case *model.OpaqueType:
		switch t {
		case model.BoolType:
			return &astproto.Type{Value: &astproto.Type_BoolType{BoolType: &astproto.Empty{}}}, nil
		case model.IntType:
			return &astproto.Type{Value: &astproto.Type_IntType{IntType: &astproto.Empty{}}}, nil
		case model.NumberType:
			return &astproto.Type{Value: &astproto.Type_NumberType{NumberType: &astproto.Empty{}}}, nil
		case model.StringType:
			return &astproto.Type{Value: &astproto.Type_StringType{StringType: &astproto.Empty{}}}, nil
		default:
			return &astproto.Type{Value: &astproto.Type_Composite{Composite: &astproto.Empty{}}}, nil
		}
	case *model.OutputType:
		elementType, err := transformProgramType(t.ElementType)
		if err != nil {
			return nil, err
		}
		return &astproto.Type{
			Value: &astproto.Type_OutputType{OutputType: elementType},
		}, nil
	default:
		return &astproto.Type{Value: &astproto.Type_Composite{Composite: &astproto.Empty{}}}, nil
	}
}

func transformTraversal(traversal hcl.Traversal) (*astproto.Traversal, error) {
	result := make([]*astproto.Traverser, len(traversal))
	for i, traverser := range traversal {
		traverser, err := transformTraverser(traverser)
		if err != nil {
			return nil, fmt.Errorf("could not transform traversal: %w", err)
		}
		result[i] = traverser
	}

	return &astproto.Traversal{
		Each: result,
	}, nil
}

func transformTraverser(part hcl.Traverser) (*astproto.Traverser, error) {
	switch part := part.(type) {
	case hcl.TraverseAttr:
		return &astproto.Traverser{
			Value: &astproto.Traverser_TraverseAttr{
				TraverseAttr: &astproto.TraverseAttr{
					Name: part.Name,
				},
			},
		}, nil
	case hcl.TraverseIndex:
		switch part.Key.Type() {
		case cty.Number:
			number, _ := part.Key.AsBigFloat().Float64()
			return &astproto.Traverser{
				Value: &astproto.Traverser_TraverseIndex{
					TraverseIndex: &astproto.TraverseIndex{
						Value: &astproto.TraverseIndex_IntIndex{
							IntIndex: int64(number),
						},
					},
				},
			}, nil
		case cty.String:
			return &astproto.Traverser{
				Value: &astproto.Traverser_TraverseIndex{
					TraverseIndex: &astproto.TraverseIndex{
						Value: &astproto.TraverseIndex_StringIndex{
							StringIndex: part.Key.AsString(),
						},
					},
				},
			}, nil
		default:
			return nil, fmt.Errorf("unknown traverse index type: %v", part.Key.Type())
		}
	case hcl.TraverseRoot:
		return &astproto.Traverser{
			Value: &astproto.Traverser_TraverseRoot{
				TraverseRoot: &astproto.TraverseRoot{
					Name: part.Name,
				},
			},
		}, nil
	case hcl.TraverseSplat:
		{
			each, err := transformTraversal(part.Each)
			if err != nil {
				return nil, fmt.Errorf("could not transform splat traversal: %w", err)
			}
			return &astproto.Traverser{
				Value: &astproto.Traverser_TraverseSplat{
					TraverseSplat: &astproto.TraverseSplat{
						Each: each,
					},
				},
			}, nil
		}
	default:
		return nil, fmt.Errorf("unknown traversal part type: %T", part)
	}
}

func transformFunctionParameters(parameters []*model.Variable) []string {
	result := make([]string, len(parameters))
	for i, parameter := range parameters {
		result[i] = parameter.Name
	}
	return result
}

func formatOperation(operation *hclsyntax.Operation) (astproto.Operation, error) {
	switch operation {
	case hclsyntax.OpAdd:
		return astproto.Operation_ADD, nil
	case hclsyntax.OpDivide:
		return astproto.Operation_DIVIDE, nil
	case hclsyntax.OpEqual:
		return astproto.Operation_EQUAL, nil
	case hclsyntax.OpGreaterThan:
		return astproto.Operation_GREATER_THAN, nil
	case hclsyntax.OpGreaterThanOrEqual:
		return astproto.Operation_GREATER_THAN_OR_EQUAL, nil
	case hclsyntax.OpLessThan:
		return astproto.Operation_LESS_THAN, nil
	case hclsyntax.OpLessThanOrEqual:
		return astproto.Operation_LESS_THAN_OR_EQUAL, nil
	case hclsyntax.OpLogicalAnd:
		return astproto.Operation_LOGICAL_AND, nil
	case hclsyntax.OpLogicalNot:
		return astproto.Operation_LOGICAL_NOT, nil
	case hclsyntax.OpLogicalOr:
		return astproto.Operation_LOGICAL_OR, nil
	case hclsyntax.OpModulo:
		return astproto.Operation_MODULO, nil
	case hclsyntax.OpMultiply:
		return astproto.Operation_MULTIPLY, nil
	case hclsyntax.OpNegate:
		return astproto.Operation_NEGATE, nil
	case hclsyntax.OpNotEqual:
		return astproto.Operation_NOT_EQUAL, nil
	case hclsyntax.OpSubtract:
		return astproto.Operation_SUBTRACT, nil
	default:
		return astproto.Operation_ADD, fmt.Errorf("unknown operation type: %v", operation)
	}
}

func transformExpression(expr model.Expression) (*astproto.Expression, error) {
	if expr == nil {
		return nil, nil
	}

	expressionType, err := transformExpressionType(expr.Type())
	if err != nil {
		return nil, fmt.Errorf("could not transform expression type for %T (%v): %w", expr, expr, err)
	}

	switch expr := expr.(type) {
	/* TODO: Support enums
	pcl resolves enums into constants on it's own. Must check how it's implemented on other languages
	*/
	case *model.LiteralValueExpression:
		var value *astproto.LiteralValueExpression
		switch expr.Value.Type() {
		case cty.Bool:
			value = &astproto.LiteralValueExpression{
				Value: &astproto.LiteralValueExpression_BoolValue{
					BoolValue: expr.Value.True(),
				},
			}
		case cty.Number:
			number, _ := expr.Value.AsBigFloat().Float64()
			value = &astproto.LiteralValueExpression{
				Value: &astproto.LiteralValueExpression_NumberValue{
					NumberValue: number,
				},
			}
		case cty.String:
			value = &astproto.LiteralValueExpression{
				Value: &astproto.LiteralValueExpression_StringValue{
					StringValue: expr.Value.AsString(),
				},
			}
		default:
			// TODO: Maybe throw error instead? Are we sure this is null?
			value = &astproto.LiteralValueExpression{Value: &astproto.LiteralValueExpression_UnknownValue{UnknownValue: true}}
		}
		return &astproto.Expression{Value: &astproto.Expression_LiteralValueExpression{LiteralValueExpression: value}, Type: expressionType}, nil

	case *model.TemplateExpression:
		parts := make([]*astproto.Expression, len(expr.Parts))
		for i, part := range expr.Parts {
			transformedPart, err := transformExpression(part)
			if err != nil {
				return nil, err
			}
			parts[i] = transformedPart
		}
		return &astproto.Expression{
			Value: &astproto.Expression_TemplateExpression{
				TemplateExpression: &astproto.TemplateExpression{Parts: parts},
			},
			Type: expressionType,
		}, nil

	case *model.IndexExpression:
		collection, err := transformExpression(expr.Collection)
		if err != nil {
			return nil, err
		}
		key, err := transformExpression(expr.Key)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_IndexExpression{
				IndexExpression: &astproto.IndexExpression{Collection: collection, Key: key},
			},
			Type: expressionType,
		}, nil

	case *model.ObjectConsExpression:
		properties := make(map[string]*astproto.Expression)
		for _, item := range expr.Items {
			key := objectKey(item)
			transformedValue, err := transformExpression(item.Value)
			if err != nil {
				return nil, fmt.Errorf("Failed to get value for key %s: %w", key, err)
			}
			properties[key] = transformedValue
		}
		return &astproto.Expression{
			Value: &astproto.Expression_ObjectConsExpression{
				ObjectConsExpression: &astproto.ObjectConsExpression{
					Properties: properties,
				},
			},
			Type: expressionType,
		}, nil

	case *model.TupleConsExpression:
		items := make([]*astproto.Expression, len(expr.Expressions))
		for i, item := range expr.Expressions {
			transformedItem, err := transformExpression(item)
			if err != nil {
				return nil, err
			}
			items[i] = transformedItem
		}
		return &astproto.Expression{
			Value: &astproto.Expression_TupleConsExpression{
				TupleConsExpression: &astproto.TupleConsExpression{
					Items: items,
				},
			},
			Type: expressionType,
		}, nil

	case *model.FunctionCallExpression:
		args := make([]*astproto.FunctionCallArgument, len(expr.Args))
		for i, arg := range expr.Args {
			transformedArg, err := transformExpression(arg)
			if err != nil {
				return nil, err
			}
			transformedType, err := transformProgramType(arg.Type())
			if err != nil {
				return nil, fmt.Errorf("could not transform function call argument type: %w", err)
			}
			args[i] = &astproto.FunctionCallArgument{
				Value: transformedArg,
				Type:  transformedType,
			}
		}
		return &astproto.Expression{
			Value: &astproto.Expression_FunctionCallExpression{
				FunctionCallExpression: &astproto.FunctionCallExpression{
					Name: expr.Name,
					Args: args,
				},
			},
			Type: expressionType,
		}, nil

	case *model.RelativeTraversalExpression:
		source, err := transformExpression(expr.Source)
		if err != nil {
			return nil, err
		}
		traversal, err := transformTraversal(expr.Traversal)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_RelativeTraversalExpression{
				RelativeTraversalExpression: &astproto.RelativeTraversalExpression{
					Source:    source,
					Traversal: traversal,
				},
			},
			Type: expressionType,
		}, nil

	case *model.ScopeTraversalExpression:
		traversal, err := transformTraversal(expr.Traversal)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_ScopeTraversalExpression{
				ScopeTraversalExpression: &astproto.ScopeTraversalExpression{
					RootName:  expr.RootName,
					Traversal: traversal,
				},
			},
			Type: expressionType,
		}, nil

	case *model.AnonymousFunctionExpression:
		body, err := transformExpression(expr.Body)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_AnonymousFunctionExpression{
				AnonymousFunctionExpression: &astproto.AnonymousFunctionExpression{
					Parameters: transformFunctionParameters(expr.Parameters),
					Body:       body,
				},
			},
			Type: expressionType,
		}, nil

	case *model.ConditionalExpression:
		condition, err := transformExpression(expr.Condition)
		if err != nil {
			return nil, err
		}
		trueExpr, err := transformExpression(expr.TrueResult)
		if err != nil {
			return nil, err
		}
		falseExpr, err := transformExpression(expr.FalseResult)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_ConditionalExpression{
				ConditionalExpression: &astproto.ConditionalExpression{
					Condition: condition,
					TrueExpr:  trueExpr,
					FalseExpr: falseExpr,
				},
			},
			Type: expressionType,
		}, nil

	case *model.BinaryOpExpression:
		left, err := transformExpression(expr.LeftOperand)
		if err != nil {
			return nil, err
		}
		right, err := transformExpression(expr.RightOperand)
		if err != nil {
			return nil, err
		}
		operation, err := formatOperation(expr.Operation)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_BinaryOpExpression{
				BinaryOpExpression: &astproto.BinaryOpExpression{
					Left:      left,
					Right:     right,
					Operation: operation,
				},
			},
			Type: expressionType,
		}, nil

	case *model.UnaryOpExpression:
		operand, err := transformExpression(expr.Operand)
		if err != nil {
			return nil, err
		}
		operation, err := formatOperation(expr.Operation)
		if err != nil {
			return nil, err
		}
		return &astproto.Expression{
			Value: &astproto.Expression_UnaryOpExpression{
				UnaryOpExpression: &astproto.UnaryOpExpression{
					Operand:   operand,
					Operation: operation,
				},
			},
			Type: expressionType,
		}, nil

	default:
		return nil, fmt.Errorf("unknown expression type: %T", expr)
	}
}

func transformResourceOptions(options *pcl.ResourceOptions) (*astproto.ResourceOptions, error) {
	optionsProto := &astproto.ResourceOptions{}

	if options.DependsOn != nil {
		dependsOn, err := transformExpression(options.DependsOn)
		if err != nil {
			return nil, err
		}
		optionsProto.DependsOn = dependsOn
	}

	if options.IgnoreChanges != nil {
		ignoreChanges, err := transformExpression(options.IgnoreChanges)
		if err != nil {
			return nil, err
		}
		optionsProto.IgnoreChanges = ignoreChanges
	}

	if options.Parent != nil {
		parent, err := transformExpression(options.Parent)
		if err != nil {
			return nil, err
		}
		optionsProto.Parent = parent
	}

	if options.Protect != nil {
		protect, err := transformExpression(options.Protect)
		if err != nil {
			return nil, err
		}
		optionsProto.Protect = protect
	}

	if options.Provider != nil {
		provider, err := transformExpression(options.Provider)
		if err != nil {
			return nil, err
		}
		optionsProto.Provider = provider
	}

	if options.Version != nil {
		version, err := transformExpression(options.Version)
		if err != nil {
			return nil, err
		}
		optionsProto.Version = version
	}

	if options.Range != nil {
		rangeExp, err := transformExpression(options.Range)
		if err != nil {
			return nil, err
		}
		optionsProto.Range = rangeExp
	}

	return optionsProto, nil
}

func transformResource(resource *pcl.Resource) (*astproto.Resource, error) {
	token := resource.Token
	if resource.Schema != nil {
		token = resource.Schema.Token // resource.Token() does not contain "index"
	}
	resourceProto := &astproto.Resource{
		Name:        resource.Name(), // It is deprecated. Should it be removed?
		Token:       token,
		LogicalName: resource.LogicalName(),
	}

	inputs := make([]*astproto.ResourceInput, len(resource.Inputs))
	for i, attr := range resource.Inputs {
		transformedValue, err := transformExpression(attr.Value)
		if err != nil {
			return nil, err
		}
		inputs[i] = &astproto.ResourceInput{
			Name:  attr.Name,
			Value: transformedValue,
		}
	}
	resourceProto.Inputs = inputs

	if resource.Options != nil {
		optionsProto, err := transformResourceOptions(resource.Options)
		if err != nil {
			return nil, err
		}
		resourceProto.Options = optionsProto
	}

	return resourceProto, nil
}

func transformLocalVariable(variable *pcl.LocalVariable) (*astproto.LocalVariable, error) {
	value, err := transformExpression(variable.Definition.Value)
	if err != nil {
		return nil, err
	}

	return &astproto.LocalVariable{
		Name:        variable.Name(),
		LogicalName: variable.LogicalName(),
		Value:       value,
	}, nil
}

func transformOutput(output *pcl.OutputVariable) (*astproto.OutputVariable, error) {
	value, err := transformExpression(output.Value)
	if err != nil {
		return nil, err
	}
	expressionType, err := transformExpressionType(output.Type())
	if err != nil {
		return nil, fmt.Errorf("could not transform expression type for output %q: %w", output.Name(), err)
	}

	return &astproto.OutputVariable{
		Name:           output.Name(),
		LogicalName:    output.LogicalName(),
		Value:          value,
		ExpressionType: expressionType,
	}, nil
}

func transformConfigType(variableType model.Type) (*astproto.ConfigType, error) {
	variableType = pcl.UnwrapOption(model.ResolveOutputs(variableType))

	switch variableType {
	case model.StringType:
		return &astproto.ConfigType{
			Value: &astproto.ConfigType_StringType{StringType: &astproto.Empty{}},
		}, nil
	case model.NumberType:
		return &astproto.ConfigType{
			Value: &astproto.ConfigType_NumberType{NumberType: &astproto.Empty{}},
		}, nil
	case model.IntType:
		return &astproto.ConfigType{
			Value: &astproto.ConfigType_IntType{IntType: &astproto.Empty{}},
		}, nil
	case model.BoolType:
		return &astproto.ConfigType{
			Value: &astproto.ConfigType_BoolType{BoolType: &astproto.Empty{}},
		}, nil
	default:
		switch variableType := variableType.(type) {
		case *model.ListType:
			elementType, err := transformConfigType(variableType.ElementType)
			if err != nil {
				return nil, err
			}
			return &astproto.ConfigType{
				Value: &astproto.ConfigType_ListType{ListType: elementType},
			}, nil
		case *model.MapType:
			elementType, err := transformConfigType(variableType.ElementType)
			if err != nil {
				return nil, err
			}
			return &astproto.ConfigType{
				Value: &astproto.ConfigType_MapType{MapType: elementType},
			}, nil
		default:
			return nil, fmt.Errorf("unknown config variable type: %v", variableType)
		}
	}
}

// transformExpressionType converts an expression's type to an ExpressionType proto message.
// Unlike transformConfigType, it does not unwrap Output types — Output<T> is represented
// as ExpressionType with an outputType variant wrapping the inner type.
func transformExpressionType(t model.Type) (*astproto.ExpressionType, error) {
	switch t {
	case model.StringType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_StringType{StringType: &astproto.Empty{}},
		}, nil
	case model.NumberType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_NumberType{NumberType: &astproto.Empty{}},
		}, nil
	case model.IntType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_IntType{IntType: &astproto.Empty{}},
		}, nil
	case model.BoolType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_BoolType{BoolType: &astproto.Empty{}},
		}, nil
	case model.DynamicType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_DynamicType{DynamicType: &astproto.Empty{}},
		}, nil
	case model.NoneType:
		return &astproto.ExpressionType{
			Value: &astproto.ExpressionType_NoneType{NoneType: &astproto.Empty{}},
		}, nil
	default:
		switch t := t.(type) {
		case *model.OutputType:
			inner, err := transformExpressionType(t.ElementType)
			if err != nil {
				return nil, fmt.Errorf("could not transform output element type: %w", err)
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_OutputType{OutputType: inner},
			}, nil
		case *model.ListType:
			inner, err := transformExpressionType(t.ElementType)
			if err != nil {
				return nil, fmt.Errorf("could not transform list element type: %w", err)
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_ListType{ListType: inner},
			}, nil
		case *model.MapType:
			inner, err := transformExpressionType(t.ElementType)
			if err != nil {
				return nil, fmt.Errorf("could not transform map element type: %w", err)
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_MapType{MapType: inner},
			}, nil
		case *model.ConstType:
			// Singleton/literal type — delegate to the underlying base type.
			// e.g. cty.StringVal("Hello") → string, cty.NumberIntVal(0) → int
			return transformExpressionType(t.Type)
		case *model.TupleType:
			elementTypes := make([]*astproto.ExpressionType, 0, len(t.ElementTypes))
			for _, elemType := range t.ElementTypes {
				transformed, err := transformExpressionType(elemType)
				if err != nil {
					return nil, fmt.Errorf("could not transform tuple element type: %w", err)
				}
				elementTypes = append(elementTypes, transformed)
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_TupleType{
					TupleType: &astproto.TupleExpressionType{
						ElementTypes: elementTypes,
					},
				},
			}, nil
		case *model.ObjectType:
			properties := make(map[string]*astproto.ExpressionType, len(t.Properties))
			for name, propType := range t.Properties {
				transformed, err := transformExpressionType(propType)
				if err != nil {
					return nil, fmt.Errorf("could not transform object property type %q: %w", name, err)
				}
				properties[name] = transformed
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_ObjectType{
					ObjectType: &astproto.ObjectExpressionType{
						Properties: properties,
					},
				},
			}, nil
		case *model.UnionType:
			elementTypes := make([]*astproto.ExpressionType, 0, len(t.ElementTypes))
			for _, elemType := range t.ElementTypes {
				transformed, err := transformExpressionType(elemType)
				if err != nil {
					return nil, fmt.Errorf("could not transform union element type: %w", err)
				}
				elementTypes = append(elementTypes, transformed)
			}
			return &astproto.ExpressionType{
				Value: &astproto.ExpressionType_UnionType{
					UnionType: &astproto.UnionExpressionType{
						ElementTypes: elementTypes,
					},
				},
			}, nil
		default:
			return nil, fmt.Errorf("unknown expression type: %T (%v)", t, t)
		}
	}
}

func transformPulumiBlock(block *pcl.PulumiBlock) (*astproto.PulumiBlock, error) {
	requiredVersionRange, err := transformExpression(block.RequiredVersion)
	if err != nil {
		return nil, fmt.Errorf("could not transform pulumi block required version: %w", err)
	}

	return &astproto.PulumiBlock{
		RequiredVersionRange: requiredVersionRange,
	}, nil
}

func transformConfigVariable(variable *pcl.ConfigVariable) (*astproto.ConfigVariable, error) {
	defaultValue, err := transformExpression(variable.DefaultValue)
	if err != nil {
		return nil, err
	}

	configType, err := transformConfigType(variable.Type())
	if err != nil {
		return nil, err
	}

	return &astproto.ConfigVariable{
		Name:         variable.Name(),
		LogicalName:  variable.LogicalName(),
		ConfigType:   configType,
		DefaultValue: defaultValue,
		Secret:       variable.Secret,
	}, nil
}

func transformProgram(pclNodes []pcl.Node, pclPackages []*schema.Package) (*astproto.PclProtobufProgram, error) {
	nodes := make([]*astproto.Node, len(pclNodes))
	plugins := make([]*astproto.PluginReference, len(pclPackages))

	for i, node := range pclNodes {
		var transformedNode *astproto.Node
		switch node := node.(type) {
		case *pcl.Resource:
			transformedResource, err := transformResource(node)
			if err != nil {
				return nil, err
			}
			transformedNode = &astproto.Node{
				Value: &astproto.Node_Resource{Resource: transformedResource},
			}
		case *pcl.OutputVariable:
			transformedOutput, err := transformOutput(node)
			if err != nil {
				return nil, err
			}
			transformedNode = &astproto.Node{
				Value: &astproto.Node_OutputVariable{OutputVariable: transformedOutput},
			}
		case *pcl.LocalVariable:
			transformedVariable, err := transformLocalVariable(node)
			if err != nil {
				return nil, err
			}
			transformedNode = &astproto.Node{
				Value: &astproto.Node_LocalVariable{LocalVariable: transformedVariable},
			}
		case *pcl.ConfigVariable:
			transformedVariable, err := transformConfigVariable(node)
			if err != nil {
				return nil, err
			}
			transformedNode = &astproto.Node{
				Value: &astproto.Node_ConfigVariable{ConfigVariable: transformedVariable},
			}
		case *pcl.PulumiBlock:
			transformedPulumiBlock, err := transformPulumiBlock(node)
			if err != nil {
				return nil, err
			}
			transformedNode = &astproto.Node{
				Value: &astproto.Node_PulumiBlock{PulumiBlock: transformedPulumiBlock},
			}
		default:
			return nil, fmt.Errorf("unknown node type type: %v", node.Type())
		}
		nodes[i] = transformedNode
	}

	for i, pkg := range pclPackages {
		version := ""
		if pkg.Version != nil {
			version = pkg.Version.String()
		}

		pluginRef := &astproto.PluginReference{
			Name:    pkg.Name,
			Version: version,
		}
		plugins[i] = pluginRef
	}

	return &astproto.PclProtobufProgram{
		Nodes:   nodes,
		Plugins: plugins,
	}, nil
}

// nameInfo implements pcl.NameInfo for RewriteApplies, returning parameter names unchanged.
type nameInfo int

func (nameInfo) Format(name string) string { return name }

// rewriteAppliesInNodes rewrites output-dependent expressions in each node so that
// any sub-expression that observes the resolved value of an Output is wrapped in a
// call to the __apply intrinsic. This mirrors what Go/Node.js/Python/Java code
// generators do via pcl.RewriteApplies before emitting code.
func rewriteAppliesInNodes(nodes []pcl.Node) {
	for _, node := range nodes {
		switch n := node.(type) {
		case *pcl.OutputVariable:
			rewritten, _ := pcl.RewriteApplies(n.Value, nameInfo(0), false)
			n.Value = rewritten
		case *pcl.LocalVariable:
			rewritten, _ := pcl.RewriteApplies(n.Definition.Value, nameInfo(0), false)
			n.Definition.Value = rewritten
		case *pcl.Resource:
			for _, input := range n.Inputs {
				rewritten, _ := pcl.RewriteApplies(input.Value, nameInfo(0), false)
				input.Value = rewritten
			}
		}
	}
}

func generateProtobufProgram(program *pcl.Program) (*astproto.PclProtobufProgram, error) {
	pcl.MapProvidersAsResources(program)
	// Linearize the nodes into an order appropriate for procedural code generation.
	nodes := pcl.Linearize(program)
	// Rewrite output-dependent expressions into __apply intrinsic calls so that
	// the Rust code generator can emit proper .map() / combineN().map() calls.
	rewriteAppliesInNodes(nodes)
	packages, err := program.PackageSnapshots()
	if err != nil {
		return nil, err
	}
	serialized, err := transformProgram(nodes, packages)
	if err != nil {
		return nil, err
	}
	return serialized, nil
}

func GenerateProtobuf(program *pcl.Program) (*astproto.PclProtobufProgram, error) {
	protobuf, err := generateProtobufProgram(program)
	if err != nil {
		return nil, fmt.Errorf("could not generate protobuf program: %w", err)
	}
	return protobuf, nil
}

func GenerateJSONProgram(program *pcl.Program) (map[string][]byte, hcl.Diagnostics, error) {
	protobuf, err := generateProtobufProgram(program)
	if err != nil {
		return nil, nil, err
	}
	bytes, err := protojson.MarshalOptions{Multiline: true}.Marshal(protobuf)
	if err != nil {
		return nil, nil, err
	}
	bytes, err = shared.NormalizeJSON(bytes)
	if err != nil {
		return nil, nil, err
	}
	return map[string][]byte{"main.pcl.json": bytes}, nil, nil
}

func GenerateSerializedProtobufProgram(program *pcl.Program) (map[string][]byte, hcl.Diagnostics, error) {
	protobuf, err := generateProtobufProgram(program)
	if err != nil {
		return nil, nil, err
	}
	out, err := proto.Marshal(protobuf)
	if err != nil {
		return nil, nil, err
	}
	str := base64.StdEncoding.EncodeToString(out)
	return map[string][]byte{"main.pcl.protobuf": []byte(str)}, nil, nil
}

func objectKey(item model.ObjectConsItem) string {
	switch key := item.Key.(type) {
	case *model.LiteralValueExpression:
		return key.Value.AsString()
	case *model.TemplateExpression:
		// assume a template expression has one constant part that is a LiteralValueExpression
		if len(key.Parts) == 1 {
			if literal, ok := key.Parts[0].(*model.LiteralValueExpression); ok {
				return literal.Value.AsString()
			}
		}
	}

	return ""
}
