package ast

import (
	"fmt"
	"math"
	"sort"
	"strings"

	"github.com/andrzejressel/pulumi-gestalt/pulumi-language-rust/ast/codegen/shared"
	astproto "github.com/andrzejressel/pulumi-gestalt/pulumi-language-rust/ast/protobuf/schemapackage"
	"github.com/hashicorp/hcl/v2"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"google.golang.org/protobuf/encoding/protojson"
)

func tokenToElementID(token string) *astproto.ElementId {
	elementID := &astproto.ElementId{
		Name: token,
		Raw:  token,
	}

	parts := strings.Split(token, ":")
	if len(parts) != 3 {
		return elementID
	}

	module := parts[1]
	name := parts[2]
	namespaces := make([]string, 0)
	for _, part := range strings.Split(module, "/") {
		if part == "" || part == "index" {
			continue
		}
		namespaces = append(namespaces, part)
	}

	elementID.Name = name
	elementID.Namespace = namespaces
	return elementID
}

func transformRefType(ref *astproto.RefType) *astproto.Type {
	return &astproto.Type{
		TypeValue: &astproto.Type_RefType{
			RefType: ref,
		},
	}
}

func toNumberEnumValue(value any) (float64, error) {
	switch value := value.(type) {
	case float64:
		return value, nil
	case float32:
		return float64(value), nil
	case int:
		return float64(value), nil
	case int64:
		return float64(value), nil
	case int32:
		return float64(value), nil
	case int16:
		return float64(value), nil
	case int8:
		return float64(value), nil
	case uint:
		return float64(value), nil
	case uint64:
		return float64(value), nil
	case uint32:
		return float64(value), nil
	case uint16:
		return float64(value), nil
	case uint8:
		return float64(value), nil
	default:
		return 0, fmt.Errorf("unsupported number enum value type: %T", value)
	}
}

func toIntegerEnumValue(value any) (int64, error) {
	switch value := value.(type) {
	case int:
		return int64(value), nil
	case int64:
		return value, nil
	case int32:
		return int64(value), nil
	case int16:
		return int64(value), nil
	case int8:
		return int64(value), nil
	case uint:
		if value > uint(math.MaxInt64) {
			return 0, fmt.Errorf("uint value %d overflows int64", value)
		}
		return int64(value), nil
	case uint64:
		if value > uint64(math.MaxInt64) {
			return 0, fmt.Errorf("uint64 value %d overflows int64", value)
		}
		return int64(value), nil
	case uint32:
		return int64(value), nil
	case uint16:
		return int64(value), nil
	case uint8:
		return int64(value), nil
	default:
		return 0, fmt.Errorf("unsupported integer enum value type: %T", value)
	}
}

func transformType(t schema.Type) (*astproto.Type, error) {
	for {
		inputType, ok := t.(*schema.InputType)
		if !ok {
			break
		}
		t = inputType.ElementType
	}

	switch t {
	case schema.BoolType:
		return &astproto.Type{
			TypeValue: &astproto.Type_BoolType{
				BoolType: &astproto.Empty{},
			},
		}, nil
	case schema.IntType:
		return &astproto.Type{
			TypeValue: &astproto.Type_IntType{
				IntType: &astproto.Empty{},
			},
		}, nil
	case schema.NumberType:
		return &astproto.Type{
			TypeValue: &astproto.Type_NumberType{
				NumberType: &astproto.Empty{},
			},
		}, nil
	case schema.StringType:
		return &astproto.Type{
			TypeValue: &astproto.Type_StringType{
				StringType: &astproto.Empty{},
			},
		}, nil
	case schema.ArchiveType:
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_Archive{
				Archive: &astproto.Empty{},
			},
		}), nil
	case schema.AssetType:
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_Asset{
				Asset: &astproto.Empty{},
			},
		}), nil
	case schema.AnyType, schema.JSONType, schema.AnyResourceType:
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_Any{
				Any: &astproto.Empty{},
			},
		}), nil
	}

	switch t := t.(type) {
	case *schema.OptionalType:
		elementType, err := transformType(t.ElementType)
		if err != nil {
			return nil, err
		}
		return &astproto.Type{
			TypeValue: &astproto.Type_OptionalType{
				OptionalType: elementType,
			},
		}, nil
	case *schema.ArrayType:
		elementType, err := transformType(t.ElementType)
		if err != nil {
			return nil, err
		}
		return &astproto.Type{
			TypeValue: &astproto.Type_ArrayType{
				ArrayType: elementType,
			},
		}, nil
	case *schema.MapType:
		elementType, err := transformType(t.ElementType)
		if err != nil {
			return nil, err
		}
		return &astproto.Type{
			TypeValue: &astproto.Type_MapType{
				MapType: elementType,
			},
		}, nil
	case *schema.UnionType:
		unionTypes := make([]*astproto.Type, len(t.ElementTypes))
		for i, elementType := range t.ElementTypes {
			converted, err := transformType(elementType)
			if err != nil {
				return nil, err
			}
			unionTypes[i] = converted
		}

		var defaultType *astproto.Type
		if t.DefaultType != nil {
			convertedDefault, err := transformType(t.DefaultType)
			if err != nil {
				return nil, err
			}
			defaultType = convertedDefault
		}

		return &astproto.Type{
			TypeValue: &astproto.Type_UnionType{
				UnionType: &astproto.DiscriminatedUnion{
					Types:         unionTypes,
					DefaultType:   defaultType,
					Discriminator: t.Discriminator,
					Mapping:       t.Mapping,
				},
			},
		}, nil
	case *schema.ObjectType:
		if t.Token == "" {
			return nil, fmt.Errorf("object type without token is not supported")
		}
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_TypeRef{
				TypeRef: tokenToElementID(t.Token),
			},
		}), nil
	case *schema.EnumType:
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_TypeRef{
				TypeRef: tokenToElementID(t.Token),
			},
		}), nil
	case *schema.ResourceType:
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_TypeRef{
				TypeRef: tokenToElementID(t.Token),
			},
		}), nil
	case *schema.TokenType:
		if t.Token == "" {
			return nil, fmt.Errorf("token type without token is not supported")
		}
		return transformRefType(&astproto.RefType{
			RefValue: &astproto.RefType_TypeRef{
				TypeRef: tokenToElementID(t.Token),
			},
		}), nil
	case *schema.InvalidType:
		return nil, fmt.Errorf("invalid schema type")
	default:
		return nil, fmt.Errorf("unsupported schema type: %T", t)
	}
}

func transformInputProperties(properties []*schema.Property) ([]*astproto.InputProperty, error) {
	sorted := append([]*schema.Property{}, properties...)
	sort.Slice(sorted, func(i, j int) bool {
		return sorted[i].Name < sorted[j].Name
	})

	result := make([]*astproto.InputProperty, len(sorted))
	for i, property := range sorted {
		propertyType, err := transformType(property.Type)
		if err != nil {
			return nil, fmt.Errorf("could not transform input property type %q: %w", property.Name, err)
		}
		result[i] = &astproto.InputProperty{
			Name:        property.Name,
			Type:        propertyType,
			Description: property.Comment,
		}
	}
	return result, nil
}

func transformOutputProperties(properties []*schema.Property) ([]*astproto.OutputProperty, error) {
	sorted := append([]*schema.Property{}, properties...)
	sort.Slice(sorted, func(i, j int) bool {
		return sorted[i].Name < sorted[j].Name
	})

	result := make([]*astproto.OutputProperty, len(sorted))
	for i, property := range sorted {
		propertyType, err := transformType(property.Type)
		if err != nil {
			return nil, fmt.Errorf("could not transform output property type %q: %w", property.Name, err)
		}
		result[i] = &astproto.OutputProperty{
			Name:        property.Name,
			Type:        propertyType,
			Description: property.Comment,
		}
	}
	return result, nil
}

func transformProvider(provider *schema.Resource) (*astproto.Provider, error) {
	if provider == nil {
		return nil, nil
	}

	inputProperties, err := transformInputProperties(provider.InputProperties)
	if err != nil {
		return nil, err
	}
	outputProperties, err := transformOutputProperties(provider.Properties)
	if err != nil {
		return nil, err
	}

	return &astproto.Provider{
		Description:      provider.Comment,
		InputProperties:  inputProperties,
		OutputProperties: outputProperties,
	}, nil
}

func transformSchemaResource(resource *schema.Resource) (*astproto.Resource, error) {
	inputProperties, err := transformInputProperties(resource.InputProperties)
	if err != nil {
		return nil, err
	}
	outputProperties, err := transformOutputProperties(resource.Properties)
	if err != nil {
		return nil, err
	}

	return &astproto.Resource{
		ElementId:        tokenToElementID(resource.Token),
		Description:      resource.Comment,
		InputProperties:  inputProperties,
		OutputProperties: outputProperties,
	}, nil
}

func transformFunction(function *schema.Function) (*astproto.Function, error) {
	inputProperties := make([]*astproto.InputProperty, 0)
	if function.Inputs != nil {
		convertedInputs, err := transformInputProperties(function.Inputs.Properties)
		if err != nil {
			return nil, err
		}
		inputProperties = convertedInputs
	}

	outputProperties := make([]*astproto.OutputProperty, 0)
	if function.Outputs != nil {
		convertedOutputs, err := transformOutputProperties(function.Outputs.Properties)
		if err != nil {
			return nil, err
		}
		outputProperties = convertedOutputs
	}

	var returnType *astproto.Type
	if function.ReturnType != nil {
		convertedReturnType, err := transformType(function.ReturnType)
		if err != nil {
			return nil, fmt.Errorf("could not transform function return type %q: %w", function.Token, err)
		}
		returnType = convertedReturnType
	}

	return &astproto.Function{
		ElementId:        tokenToElementID(function.Token),
		Description:      function.Comment,
		InputProperties:  inputProperties,
		OutputProperties: outputProperties,
		ReturnType:       returnType,
	}, nil
}

func transformObjectType(objectType *schema.ObjectType) (*astproto.GlobalType, error) {
	properties := make([]*astproto.GlobalTypeProperty, len(objectType.Properties))
	for i, property := range objectType.Properties {
		propertyType, err := transformType(property.Type)
		if err != nil {
			return nil, fmt.Errorf("could not transform object property type %q: %w", property.Name, err)
		}
		properties[i] = &astproto.GlobalTypeProperty{
			Name:        property.Name,
			Type:        propertyType,
			Description: property.Comment,
		}
	}
	sort.Slice(properties, func(i, j int) bool {
		return properties[i].Name < properties[j].Name
	})

	return &astproto.GlobalType{
		ElementId: tokenToElementID(objectType.Token),
		GlobalTypeValue: &astproto.GlobalTypeValue{
			Value: &astproto.GlobalTypeValue_Object{
				Object: &astproto.ObjectType{
					Description: objectType.Comment,
					Properties:  properties,
				},
			},
		},
	}, nil
}

func transformEnumType(enumType *schema.EnumType) (*astproto.GlobalType, error) {
	switch enumType.ElementType {
	case schema.StringType:
		elements := make([]*astproto.StringEnumElement, len(enumType.Elements))
		for i, element := range enumType.Elements {
			value, ok := element.Value.(string)
			if !ok {
				return nil, fmt.Errorf("string enum %q has non-string value type %T", enumType.Token, element.Value)
			}
			elements[i] = &astproto.StringEnumElement{
				Name:        element.Name,
				Value:       value,
				Description: element.Comment,
			}
		}
		return &astproto.GlobalType{
			ElementId: tokenToElementID(enumType.Token),
			GlobalTypeValue: &astproto.GlobalTypeValue{
				Value: &astproto.GlobalTypeValue_StringEnum{
					StringEnum: &astproto.StringEnum{
						Description: enumType.Comment,
						Elements:    elements,
					},
				},
			},
		}, nil
	case schema.NumberType:
		elements := make([]*astproto.NumberEnumElement, len(enumType.Elements))
		for i, element := range enumType.Elements {
			value, err := toNumberEnumValue(element.Value)
			if err != nil {
				return nil, fmt.Errorf("number enum %q: %w", enumType.Token, err)
			}
			elements[i] = &astproto.NumberEnumElement{
				Name:        element.Name,
				Value:       value,
				Description: element.Comment,
			}
		}
		return &astproto.GlobalType{
			ElementId: tokenToElementID(enumType.Token),
			GlobalTypeValue: &astproto.GlobalTypeValue{
				Value: &astproto.GlobalTypeValue_NumberEnum{
					NumberEnum: &astproto.NumberEnum{
						Description: enumType.Comment,
						Elements:    elements,
					},
				},
			},
		}, nil
	case schema.IntType:
		elements := make([]*astproto.IntegerEnumElement, len(enumType.Elements))
		for i, element := range enumType.Elements {
			value, err := toIntegerEnumValue(element.Value)
			if err != nil {
				return nil, fmt.Errorf("integer enum %q: %w", enumType.Token, err)
			}
			elements[i] = &astproto.IntegerEnumElement{
				Name:        element.Name,
				Value:       value,
				Description: element.Comment,
			}
		}
		return &astproto.GlobalType{
			ElementId: tokenToElementID(enumType.Token),
			GlobalTypeValue: &astproto.GlobalTypeValue{
				Value: &astproto.GlobalTypeValue_IntegerEnum{
					IntegerEnum: &astproto.IntegerEnum{
						Description: enumType.Comment,
						Elements:    elements,
					},
				},
			},
		}, nil
	default:
		return nil, fmt.Errorf("unsupported enum element type for %q", enumType.Token)
	}
}

func transformGlobalTypes(types []schema.Type) ([]*astproto.GlobalType, error) {
	sorted := append([]schema.Type{}, types...)
	sort.Slice(sorted, func(i, j int) bool {
		return sorted[i].String() < sorted[j].String()
	})

	result := make([]*astproto.GlobalType, 0)
	for _, typ := range sorted {
		switch typ := typ.(type) {
		case *schema.ObjectType:
			if typ.IsInputShape() {
				continue
			}
			globalType, err := transformObjectType(typ)
			if err != nil {
				return nil, err
			}
			result = append(result, globalType)
		case *schema.EnumType:
			globalType, err := transformEnumType(typ)
			if err != nil {
				return nil, err
			}
			result = append(result, globalType)
		}
	}

	sort.Slice(result, func(i, j int) bool {
		return result[i].ElementId.Raw < result[j].ElementId.Raw
	})
	return result, nil
}

func GenerateProtobufPackage(pkg *schema.Package) (*astproto.Package, error) {
	if pkg == nil {
		return nil, fmt.Errorf("package is nil")
	}

	provider, err := transformProvider(pkg.Provider)
	if err != nil {
		return nil, fmt.Errorf("could not transform provider: %w", err)
	}

	resources := append([]*schema.Resource{}, pkg.Resources...)
	sort.Slice(resources, func(i, j int) bool {
		return resources[i].Token < resources[j].Token
	})

	protobufResources := make([]*astproto.Resource, len(resources))
	for i, resource := range resources {
		convertedResource, err := transformSchemaResource(resource)
		if err != nil {
			return nil, fmt.Errorf("could not transform resource %q: %w", resource.Token, err)
		}
		protobufResources[i] = convertedResource
	}

	functions := append([]*schema.Function{}, pkg.Functions...)
	sort.Slice(functions, func(i, j int) bool {
		return functions[i].Token < functions[j].Token
	})

	protobufFunctions := make([]*astproto.Function, len(functions))
	for i, function := range functions {
		convertedFunction, err := transformFunction(function)
		if err != nil {
			return nil, fmt.Errorf("could not transform function %q: %w", function.Token, err)
		}
		protobufFunctions[i] = convertedFunction
	}

	protobufTypes, err := transformGlobalTypes(pkg.Types)
	if err != nil {
		return nil, fmt.Errorf("could not transform global types: %w", err)
	}

	version := ""
	if pkg.Version != nil {
		version = pkg.Version.String()
	}

	return &astproto.Package{
		Name:              pkg.Name,
		DisplayName:       pkg.DisplayName,
		PluginDownloadUrl: pkg.PluginDownloadURL,
		Version:           version,
		Resources:         protobufResources,
		Functions:         protobufFunctions,
		Types:             protobufTypes,
		Provider:          provider,
	}, nil
}

func GenerateJSONPackage(pkg *schema.Package) (map[string][]byte, hcl.Diagnostics, error) {
	protobuf, err := GenerateProtobufPackage(pkg)
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

	return map[string][]byte{"package.pcl.json": bytes}, nil, nil
}
