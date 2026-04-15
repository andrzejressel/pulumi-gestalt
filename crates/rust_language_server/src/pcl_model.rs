use pulumi_gestalt_proto::language_server::pulumipcl as pb;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct PclProtobufProgram {
    pub nodes: Vec<Node>,
    pub plugins: Vec<PluginReference>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PluginReference {
    pub name: String,
    pub version: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub value: node::Value,
}

pub mod node {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Value {
        Resource(super::Resource),
        LocalVariable(super::LocalVariable),
        ConfigVariable(super::ConfigVariable),
        OutputVariable(super::OutputVariable),
        PulumiBlock(super::PulumiBlock),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Resource {
    pub name: String,
    pub logical_name: String,
    pub token: String,
    pub inputs: Vec<ResourceInput>,
    pub options: Option<ResourceOptions>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResourceInput {
    pub name: String,
    pub value: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResourceOptions {
    pub depends_on: Option<Expression>,
    pub protect: Option<Expression>,
    pub parent: Option<Expression>,
    pub ignore_changes: Option<Expression>,
    pub provider: Option<Expression>,
    pub version: Option<Expression>,
    pub range: Option<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LocalVariable {
    pub name: String,
    pub logical_name: String,
    pub value: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConfigVariable {
    pub name: String,
    pub logical_name: String,
    pub config_type: ConfigType,
    pub default_value: Option<Expression>,
    pub secret: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct OutputVariable {
    pub name: String,
    pub logical_name: String,
    pub value: Expression,
    pub expression_type: Option<ExpressionType>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PulumiBlock {
    pub required_version_range: Option<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    pub value: expression::Value,
    pub expression_type: Option<ExpressionType>,
}

pub mod expression {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Value {
        LiteralValueExpression(super::LiteralValueExpression),
        TemplateExpression(super::TemplateExpression),
        IndexExpression(Box<super::IndexExpression>),
        ObjectConsExpression(super::ObjectConsExpression),
        TupleConsExpression(super::TupleConsExpression),
        FunctionCallExpression(super::FunctionCallExpression),
        RelativeTraversalExpression(Box<super::RelativeTraversalExpression>),
        ScopeTraversalExpression(super::ScopeTraversalExpression),
        AnonymousFunctionExpression(Box<super::AnonymousFunctionExpression>),
        ConditionalExpression(Box<super::ConditionalExpression>),
        BinaryOpExpression(Box<super::BinaryOpExpression>),
        UnaryOpExpression(Box<super::UnaryOpExpression>),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct LiteralValueExpression {
    pub value: literal_value_expression::Value,
}

pub mod literal_value_expression {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Value {
        UnknownValue(bool),
        StringValue(String),
        NumberValue(f64),
        BoolValue(bool),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TemplateExpression {
    pub parts: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IndexExpression {
    pub collection: Box<Expression>,
    pub key: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObjectConsExpression {
    pub properties: HashMap<String, Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TupleConsExpression {
    pub items: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionCallExpression {
    pub name: String,
    pub args: Vec<FunctionCallArgument>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionCallArgument {
    pub value: Expression,
    pub r#type: Type,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Type {
    pub value: r#type::Value,
}

pub mod r#type {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Value {
        BoolType(super::Empty),
        IntType(super::Empty),
        NumberType(super::Empty),
        StringType(super::Empty),
        OutputType(Box<super::Type>),
        Composite(super::Empty),
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Empty {}

#[derive(Clone, PartialEq, Debug)]
pub struct RelativeTraversalExpression {
    pub source: Box<Expression>,
    pub traversal: Traversal,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScopeTraversalExpression {
    pub root_name: String,
    pub traversal: Traversal,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnonymousFunctionExpression {
    pub body: Box<Expression>,
    pub parameters: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub true_expr: Box<Expression>,
    pub false_expr: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryOpExpression {
    pub operation: Operation,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryOpExpression {
    pub operation: Operation,
    pub operand: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Traversal {
    pub each: Vec<Traverser>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Traverser {
    pub value: traverser::Value,
}

pub mod traverser {
    #[derive(Clone, PartialEq, Debug)]
    pub enum Value {
        TraverseAttr(super::TraverseAttr),
        TraverseIndex(super::TraverseIndex),
        TraverseRoot(super::TraverseRoot),
        TraverseSplat(super::TraverseSplat),
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TraverseAttr {
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TraverseIndex {
    pub value: traverse_index::Value,
}

pub mod traverse_index {
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub enum Value {
        IntIndex(i64),
        StringIndex(String),
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TraverseRoot {
    pub name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TraverseSplat {
    pub each: Traversal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfigType {
    String,
    Number,
    Int,
    Bool,
    List(Box<ConfigType>),
    Map(Box<ConfigType>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExpressionType {
    String,
    Number,
    Int,
    Bool,
    Dynamic,
    List(Box<ExpressionType>),
    Map(Box<ExpressionType>),
    Output(Box<ExpressionType>),
    Tuple(Vec<ExpressionType>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operation {
    Add,
    Divide,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    Modulo,
    Multiply,
    NotEqual,
    Subtract,
    LogicalNot,
    Negate,
}

pub fn map_program(program: pb::PclProtobufProgram) -> PclProtobufProgram {
    PclProtobufProgram {
        nodes: program.nodes.into_iter().map(map_node).collect(),
        plugins: program
            .plugins
            .into_iter()
            .map(map_plugin_reference)
            .collect(),
    }
}

fn map_plugin_reference(plugin: pb::PluginReference) -> PluginReference {
    PluginReference {
        name: plugin.name,
        version: plugin.version,
    }
}

fn map_node(node: pb::Node) -> Node {
    Node {
        value: map_node_value(required(node.value, "node.value")),
    }
}

fn map_node_value(value: pb::node::Value) -> node::Value {
    match value {
        pb::node::Value::Resource(resource) => node::Value::Resource(map_resource(resource)),
        pb::node::Value::LocalVariable(local) => {
            node::Value::LocalVariable(map_local_variable(local))
        }
        pb::node::Value::ConfigVariable(config) => {
            node::Value::ConfigVariable(map_config_variable(config))
        }
        pb::node::Value::OutputVariable(output) => {
            node::Value::OutputVariable(map_output_variable(output))
        }
        pb::node::Value::PulumiBlock(block) => node::Value::PulumiBlock(map_pulumi_block(block)),
    }
}

fn map_resource(resource: pb::Resource) -> Resource {
    Resource {
        name: resource.name,
        logical_name: resource.logical_name,
        token: resource.token,
        inputs: resource
            .inputs
            .into_iter()
            .map(map_resource_input)
            .collect(),
        options: resource.options.map(map_resource_options),
    }
}

fn map_resource_input(input: pb::ResourceInput) -> ResourceInput {
    ResourceInput {
        name: input.name,
        value: map_expression(required(input.value, "resource_input.value")),
    }
}

fn map_resource_options(options: pb::ResourceOptions) -> ResourceOptions {
    ResourceOptions {
        depends_on: options.depends_on.map(map_expression),
        protect: options.protect.map(map_expression),
        parent: options.parent.map(map_expression),
        ignore_changes: options.ignore_changes.map(map_expression),
        provider: options.provider.map(map_expression),
        version: options.version.map(map_expression),
        range: options.range.map(map_expression),
    }
}

fn map_local_variable(local: pb::LocalVariable) -> LocalVariable {
    LocalVariable {
        name: local.name,
        logical_name: local.logical_name,
        value: map_expression(required(local.value, "local_variable.value")),
    }
}

fn map_config_variable(config: pb::ConfigVariable) -> ConfigVariable {
    ConfigVariable {
        name: config.name,
        logical_name: config.logical_name,
        config_type: map_config_type(required(config.config_type, "config_variable.config_type")),
        default_value: config.default_value.map(map_expression),
        secret: config.secret,
    }
}

fn map_output_variable(output: pb::OutputVariable) -> OutputVariable {
    OutputVariable {
        name: output.name,
        logical_name: output.logical_name,
        value: map_expression(required(output.value, "output_variable.value")),
        expression_type: output.expression_type.map(map_expression_type),
    }
}

fn map_pulumi_block(block: pb::PulumiBlock) -> PulumiBlock {
    PulumiBlock {
        required_version_range: block.required_version_range.map(map_expression),
    }
}

fn map_expression(expression: pb::Expression) -> Expression {
    Expression {
        value: map_expression_value(required(expression.value, "expression.value")),
        expression_type: expression.r#type.map(map_expression_type),
    }
}

fn map_expression_value(value: pb::expression::Value) -> expression::Value {
    match value {
        pb::expression::Value::LiteralValueExpression(v) => {
            expression::Value::LiteralValueExpression(map_literal_value_expression(v))
        }
        pb::expression::Value::TemplateExpression(v) => {
            expression::Value::TemplateExpression(map_template_expression(v))
        }
        pb::expression::Value::IndexExpression(v) => {
            expression::Value::IndexExpression(Box::new(map_index_expression(*v)))
        }
        pb::expression::Value::ObjectConsExpression(v) => {
            expression::Value::ObjectConsExpression(map_object_cons_expression(v))
        }
        pb::expression::Value::TupleConsExpression(v) => {
            expression::Value::TupleConsExpression(map_tuple_cons_expression(v))
        }
        pb::expression::Value::FunctionCallExpression(v) => {
            expression::Value::FunctionCallExpression(map_function_call_expression(v))
        }
        pb::expression::Value::RelativeTraversalExpression(v) => {
            expression::Value::RelativeTraversalExpression(Box::new(
                map_relative_traversal_expression(*v),
            ))
        }
        pb::expression::Value::ScopeTraversalExpression(v) => {
            expression::Value::ScopeTraversalExpression(map_scope_traversal_expression(v))
        }
        pb::expression::Value::AnonymousFunctionExpression(v) => {
            expression::Value::AnonymousFunctionExpression(Box::new(
                map_anonymous_function_expression(*v),
            ))
        }
        pb::expression::Value::ConditionalExpression(v) => {
            expression::Value::ConditionalExpression(Box::new(map_conditional_expression(*v)))
        }
        pb::expression::Value::BinaryOpExpression(v) => {
            expression::Value::BinaryOpExpression(Box::new(map_binary_op_expression(*v)))
        }
        pb::expression::Value::UnaryOpExpression(v) => {
            expression::Value::UnaryOpExpression(Box::new(map_unary_op_expression(*v)))
        }
    }
}

fn map_literal_value_expression(value: pb::LiteralValueExpression) -> LiteralValueExpression {
    LiteralValueExpression {
        value: map_literal_value(required(value.value, "literal_value_expression.value")),
    }
}

fn map_literal_value(
    value: pb::literal_value_expression::Value,
) -> literal_value_expression::Value {
    match value {
        pb::literal_value_expression::Value::UnknownValue(v) => {
            literal_value_expression::Value::UnknownValue(v)
        }
        pb::literal_value_expression::Value::StringValue(v) => {
            literal_value_expression::Value::StringValue(v)
        }
        pb::literal_value_expression::Value::NumberValue(v) => {
            literal_value_expression::Value::NumberValue(v)
        }
        pb::literal_value_expression::Value::BoolValue(v) => {
            literal_value_expression::Value::BoolValue(v)
        }
    }
}

fn map_template_expression(value: pb::TemplateExpression) -> TemplateExpression {
    TemplateExpression {
        parts: value.parts.into_iter().map(map_expression).collect(),
    }
}

fn map_index_expression(value: pb::IndexExpression) -> IndexExpression {
    IndexExpression {
        collection: map_required_boxed_expression(value.collection, "index_expression.collection"),
        key: map_required_boxed_expression(value.key, "index_expression.key"),
    }
}

fn map_object_cons_expression(value: pb::ObjectConsExpression) -> ObjectConsExpression {
    ObjectConsExpression {
        properties: value
            .properties
            .into_iter()
            .map(|(name, expr)| (name, map_expression(expr)))
            .collect(),
    }
}

fn map_tuple_cons_expression(value: pb::TupleConsExpression) -> TupleConsExpression {
    TupleConsExpression {
        items: value.items.into_iter().map(map_expression).collect(),
    }
}

fn map_function_call_expression(value: pb::FunctionCallExpression) -> FunctionCallExpression {
    FunctionCallExpression {
        name: value.name,
        args: value
            .args
            .into_iter()
            .map(map_function_call_argument)
            .collect(),
    }
}

fn map_function_call_argument(value: pb::FunctionCallArgument) -> FunctionCallArgument {
    FunctionCallArgument {
        value: map_expression(required(value.value, "function_call_argument.value")),
        r#type: map_type(required(value.r#type, "function_call_argument.type")),
    }
}

fn map_type(value: pb::Type) -> Type {
    Type {
        value: map_type_value(required(value.value, "type.value")),
    }
}

fn map_type_value(value: pb::r#type::Value) -> r#type::Value {
    match value {
        pb::r#type::Value::BoolType(_) => r#type::Value::BoolType(Empty {}),
        pb::r#type::Value::IntType(_) => r#type::Value::IntType(Empty {}),
        pb::r#type::Value::NumberType(_) => r#type::Value::NumberType(Empty {}),
        pb::r#type::Value::StringType(_) => r#type::Value::StringType(Empty {}),
        pb::r#type::Value::OutputType(inner) => {
            r#type::Value::OutputType(Box::new(map_type(*inner)))
        }
        pb::r#type::Value::Composite(_) => r#type::Value::Composite(Empty {}),
    }
}

fn map_relative_traversal_expression(
    value: pb::RelativeTraversalExpression,
) -> RelativeTraversalExpression {
    RelativeTraversalExpression {
        source: map_required_boxed_expression(value.source, "relative_traversal_expression.source"),
        traversal: map_traversal(required(
            value.traversal,
            "relative_traversal_expression.traversal",
        )),
    }
}

fn map_scope_traversal_expression(value: pb::ScopeTraversalExpression) -> ScopeTraversalExpression {
    ScopeTraversalExpression {
        root_name: value.root_name,
        traversal: map_traversal(required(
            value.traversal,
            "scope_traversal_expression.traversal",
        )),
    }
}

fn map_anonymous_function_expression(
    value: pb::AnonymousFunctionExpression,
) -> AnonymousFunctionExpression {
    AnonymousFunctionExpression {
        body: map_required_boxed_expression(value.body, "anonymous_function_expression.body"),
        parameters: value.parameters,
    }
}

fn map_conditional_expression(value: pb::ConditionalExpression) -> ConditionalExpression {
    ConditionalExpression {
        condition: map_required_boxed_expression(
            value.condition,
            "conditional_expression.condition",
        ),
        true_expr: map_required_boxed_expression(
            value.true_expr,
            "conditional_expression.true_expr",
        ),
        false_expr: map_required_boxed_expression(
            value.false_expr,
            "conditional_expression.false_expr",
        ),
    }
}

fn map_binary_op_expression(value: pb::BinaryOpExpression) -> BinaryOpExpression {
    BinaryOpExpression {
        operation: map_operation(value.operation),
        left: map_required_boxed_expression(value.left, "binary_op_expression.left"),
        right: map_required_boxed_expression(value.right, "binary_op_expression.right"),
    }
}

fn map_unary_op_expression(value: pb::UnaryOpExpression) -> UnaryOpExpression {
    UnaryOpExpression {
        operation: map_operation(value.operation),
        operand: map_required_boxed_expression(value.operand, "unary_op_expression.operand"),
    }
}

fn map_traversal(value: pb::Traversal) -> Traversal {
    Traversal {
        each: value.each.into_iter().map(map_traverser).collect(),
    }
}

fn map_traverser(value: pb::Traverser) -> Traverser {
    Traverser {
        value: map_traverser_value(required(value.value, "traverser.value")),
    }
}

fn map_traverser_value(value: pb::traverser::Value) -> traverser::Value {
    match value {
        pb::traverser::Value::TraverseAttr(v) => {
            traverser::Value::TraverseAttr(map_traverse_attr(v))
        }
        pb::traverser::Value::TraverseIndex(v) => {
            traverser::Value::TraverseIndex(map_traverse_index(v))
        }
        pb::traverser::Value::TraverseRoot(v) => {
            traverser::Value::TraverseRoot(map_traverse_root(v))
        }
        pb::traverser::Value::TraverseSplat(v) => {
            traverser::Value::TraverseSplat(map_traverse_splat(v))
        }
    }
}

fn map_traverse_attr(value: pb::TraverseAttr) -> TraverseAttr {
    TraverseAttr { name: value.name }
}

fn map_traverse_index(value: pb::TraverseIndex) -> TraverseIndex {
    TraverseIndex {
        value: map_traverse_index_value(required(value.value, "traverse_index.value")),
    }
}

fn map_traverse_index_value(value: pb::traverse_index::Value) -> traverse_index::Value {
    match value {
        pb::traverse_index::Value::IntIndex(v) => traverse_index::Value::IntIndex(v),
        pb::traverse_index::Value::StringIndex(v) => traverse_index::Value::StringIndex(v),
    }
}

fn map_traverse_root(value: pb::TraverseRoot) -> TraverseRoot {
    TraverseRoot { name: value.name }
}

fn map_traverse_splat(value: pb::TraverseSplat) -> TraverseSplat {
    TraverseSplat {
        each: map_traversal(required(value.each, "traverse_splat.each")),
    }
}

fn map_config_type(value: pb::ConfigType) -> ConfigType {
    match required(value.value, "config_type.value") {
        pb::config_type::Value::StringType(_) => ConfigType::String,
        pb::config_type::Value::NumberType(_) => ConfigType::Number,
        pb::config_type::Value::IntType(_) => ConfigType::Int,
        pb::config_type::Value::BoolType(_) => ConfigType::Bool,
        pb::config_type::Value::ListType(v) => ConfigType::List(Box::new(map_config_type(*v))),
        pb::config_type::Value::MapType(v) => ConfigType::Map(Box::new(map_config_type(*v))),
    }
}

fn map_expression_type(value: pb::ExpressionType) -> ExpressionType {
    match required(value.value, "expression_type.value") {
        pb::expression_type::Value::StringType(_) => ExpressionType::String,
        pb::expression_type::Value::NumberType(_) => ExpressionType::Number,
        pb::expression_type::Value::IntType(_) => ExpressionType::Int,
        pb::expression_type::Value::BoolType(_) => ExpressionType::Bool,
        pb::expression_type::Value::ListType(v) => {
            ExpressionType::List(Box::new(map_expression_type(*v)))
        }
        pb::expression_type::Value::MapType(v) => {
            ExpressionType::Map(Box::new(map_expression_type(*v)))
        }
        pb::expression_type::Value::OutputType(v) => {
            ExpressionType::Output(Box::new(map_expression_type(*v)))
        }
        pb::expression_type::Value::TupleType(v) => ExpressionType::Tuple(
            v.element_types
                .into_iter()
                .map(map_expression_type)
                .collect(),
        ),
        pb::expression_type::Value::DynamicType(_) => ExpressionType::Dynamic,
    }
}

fn map_operation(value: i32) -> Operation {
    match pb::Operation::try_from(value).unwrap_or_else(|_| panic!("operation is invalid: {value}"))
    {
        pb::Operation::Add => Operation::Add,
        pb::Operation::Divide => Operation::Divide,
        pb::Operation::Equal => Operation::Equal,
        pb::Operation::GreaterThan => Operation::GreaterThan,
        pb::Operation::GreaterThanOrEqual => Operation::GreaterThanOrEqual,
        pb::Operation::LessThan => Operation::LessThan,
        pb::Operation::LessThanOrEqual => Operation::LessThanOrEqual,
        pb::Operation::LogicalAnd => Operation::LogicalAnd,
        pb::Operation::LogicalOr => Operation::LogicalOr,
        pb::Operation::Modulo => Operation::Modulo,
        pb::Operation::Multiply => Operation::Multiply,
        pb::Operation::NotEqual => Operation::NotEqual,
        pb::Operation::Subtract => Operation::Subtract,
        pb::Operation::LogicalNot => Operation::LogicalNot,
        pb::Operation::Negate => Operation::Negate,
    }
}

fn required<T>(value: Option<T>, field: &str) -> T {
    value.unwrap_or_else(|| panic!("{field} is required"))
}

fn map_required_boxed_expression(
    value: Option<Box<pb::Expression>>,
    field: &str,
) -> Box<Expression> {
    Box::new(map_expression(*required(value, field)))
}
