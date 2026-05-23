package ast

import (
	"testing"

	"github.com/pulumi/pulumi/pkg/v3/codegen/hcl2/model"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"github.com/zclconf/go-cty/cty"
)

func TestTransformResourceInputWrapsObjectAsNewPackageType(t *testing.T) {
	nestedSchema := &schema.ObjectType{
		Token: "pkg:index:NestedType",
		Properties: []*schema.Property{
			{Name: "name", Type: schema.StringType},
		},
	}
	topSchema := &schema.ObjectType{
		Token: "pkg:index:TopType",
		Properties: []*schema.Property{
			{Name: "nested", Type: &schema.InputType{ElementType: nestedSchema}},
		},
	}

	inputExpr := objectCons(item("nested", objectCons(item("name", str("demo")))))
	transformed, err := transformResourceInputExpression(inputExpr, topSchema, "resource.res.props")
	if err != nil {
		t.Fatalf("transformResourceInputExpression failed: %v", err)
	}

	root := transformed.GetNewPackageTypeExpression()
	if root == nil {
		t.Fatalf("expected root newPackageTypeExpression")
	}
	if root.Token != "pkg:index:TopType" {
		t.Fatalf("unexpected root token: %q", root.Token)
	}

	nested := root.Properties["nested"].GetNewPackageTypeExpression()
	if nested == nil {
		t.Fatalf("expected nested newPackageTypeExpression")
	}
	if nested.Token != "pkg:index:NestedType" {
		t.Fatalf("unexpected nested token: %q", nested.Token)
	}
}

func TestTransformResourceRecursesArrayAndMapValues(t *testing.T) {
	elementSchema := &schema.ObjectType{
		Token: "pkg:index:ElementType",
		Properties: []*schema.Property{
			{Name: "value", Type: schema.StringType},
		},
	}
	mapElementSchema := &schema.ObjectType{
		Token: "pkg:index:MapValueType",
		Properties: []*schema.Property{
			{Name: "count", Type: schema.IntType},
		},
	}
	topSchema := &schema.ObjectType{
		Token: "pkg:index:TopType",
		Properties: []*schema.Property{
			{Name: "items", Type: &schema.ArrayType{ElementType: elementSchema}},
			{Name: "tags", Type: &schema.MapType{ElementType: mapElementSchema}},
		},
	}

	inputExpr := objectCons(
		item("items", tupleCons(objectCons(item("value", str("one"))))),
		item("tags", objectCons(item("a", objectCons(item("count", number(1)))))),
	)
	transformed, err := transformResourceInputExpression(inputExpr, topSchema, "resource.res.props")
	if err != nil {
		t.Fatalf("transformResourceInputExpression failed: %v", err)
	}

	root := transformed.GetNewPackageTypeExpression()
	if root == nil {
		t.Fatalf("expected root newPackageTypeExpression")
	}

	itemsTuple := root.Properties["items"].GetTupleConsExpression()
	if itemsTuple == nil {
		t.Fatalf("expected tuple for items")
	}
	item0 := itemsTuple.Items[0].GetNewPackageTypeExpression()
	if item0 == nil || item0.Token != "pkg:index:ElementType" {
		t.Fatalf("expected array element to be wrapped with ElementType token")
	}

	tagsMap := root.Properties["tags"].GetCreateMapExpression()
	if tagsMap == nil {
		t.Fatalf("expected createMapExpression for tags map literal")
	}
	tagA := tagsMap.Properties["a"].GetNewPackageTypeExpression()
	if tagA == nil || tagA.Token != "pkg:index:MapValueType" {
		t.Fatalf("expected map value to be wrapped with MapValueType token")
	}
}

func TestTransformResourceInputUnionTypeReturnsError(t *testing.T) {
	inputExpr := objectCons(item("value", str("x")))
	_, err := transformResourceInputExpression(
		inputExpr,
		&schema.UnionType{ElementTypes: []schema.Type{schema.StringType, schema.IntType}},
		"resource.test.props",
	)
	if err == nil {
		t.Fatalf("expected error for union schema type")
	}
}

func TestTransformResourceInputWithoutSchemaTypeFallsBackToObjectCons(t *testing.T) {
	inputExpr := objectCons(item("name", str("demo")))
	transformed, err := transformResourceInputExpression(
		inputExpr,
		nil,
		"resource.res.props",
	)
	if err != nil {
		t.Fatalf("transformResourceInputExpression failed: %v", err)
	}
	if transformed.GetNewPackageTypeExpression() != nil {
		t.Fatalf("expected fallback object expression when schema is missing")
	}
	if transformed.GetObjectConsExpression() == nil {
		t.Fatalf("expected objectConsExpression fallback")
	}
}

func TestTypedResourcePropertiesUnwrapsInputAndOptional(t *testing.T) {
	resourceSchema := &schema.Resource{
		Token: "pkg:index:Res",
		InputProperties: []*schema.Property{
			{
				Name: "props",
				Type: &schema.OptionalType{
					ElementType: &schema.InputType{
						ElementType: &schema.ObjectType{Token: "pkg:index:Obj"},
					},
				},
			},
		},
	}
	resource := &pcl.Resource{
		Definition: &model.Block{Labels: []string{"res"}},
		Schema:     resourceSchema,
	}
	properties := typedResourceProperties(resource)
	objType, ok := properties["props"].(*schema.ObjectType)
	if !ok {
		t.Fatalf("expected props to unwrap to object type")
	}
	if objType.Token != "pkg:index:Obj" {
		t.Fatalf("unexpected object token: %q", objType.Token)
	}
}

func item(key string, value model.Expression) model.ObjectConsItem {
	return model.ObjectConsItem{
		Key:   str(key),
		Value: value,
	}
}

func objectCons(items ...model.ObjectConsItem) *model.ObjectConsExpression {
	expr := &model.ObjectConsExpression{Items: items}
	expr.Typecheck(true)
	return expr
}

func tupleCons(items ...model.Expression) *model.TupleConsExpression {
	expr := &model.TupleConsExpression{Expressions: items}
	expr.Typecheck(true)
	return expr
}

func str(value string) *model.LiteralValueExpression {
	expr := &model.LiteralValueExpression{Value: cty.StringVal(value)}
	expr.Typecheck(false)
	return expr
}

func number(value int64) *model.LiteralValueExpression {
	expr := &model.LiteralValueExpression{Value: cty.NumberIntVal(value)}
	expr.Typecheck(false)
	return expr
}
