/// Lowers the PCL IR into the Domain IR.
///
/// This transform resolves PCL-level constructs (protobuf-shaped expressions,
/// `__apply` intrinsics, stdlib function names) into high-level Pulumi domain
/// concepts (`OutputMap`, `CombineOutputs`, `StdlibCall`, etc.).
use crate::domain_ir::{
    BinOp, ConfigBinding, ConfigType, Expr, JsonValue, Program, ResourceInput, ResourceToken,
    Statement, StdlibFn, UnaryOp,
};
use crate::pcl_model::node::Value;
use crate::pcl_model::r#type::Value::OutputType;
use crate::pcl_model::{
    self, ConfigVariable, Expression, FunctionCallExpression, LocalVariable, Node, OutputVariable,
    PclProtobufProgram, PulumiBlock, Resource, TemplateExpression, TupleConsExpression, expression,
    literal_value_expression, traverse_index, traverser,
};
use pulumi_gestalt_schema::model::ElementId;
use rootcause::compat::IntoRootcause;
use rootcause::option_ext::OptionExt;
use rootcause::prelude::ResultExt;
use rootcause::{Result, bail};

pub fn lower(program: &PclProtobufProgram) -> Result<Program> {
    let statements = program
        .nodes
        .iter()
        .map(lower_node)
        .collect::<Result<Vec<_>>>()
        .context("Failed to lower program nodes")?;

    Ok(Program { statements })
}

fn lower_node(node: &Node) -> Result<Statement> {
    match &node.value {
        Value::Resource(resource) => {
            Ok(lower_resource(resource).context("Failed to lower resource")?)
        }
        Value::LocalVariable(local) => {
            Ok(lower_local_variable(local).context("Failed to lower local variable")?)
        }
        Value::ConfigVariable(config) => {
            Ok(lower_config_variable(config).context("Failed to lower config variable")?)
        }
        Value::OutputVariable(output) => {
            Ok(lower_output_variable(output).context("Failed to lower output variable")?)
        }
        Value::PulumiBlock(block) => {
            Ok(lower_pulumi_block(block).context("Failed to lower pulumi block")?)
        }
    }
}

fn lower_local_variable(local: &LocalVariable) -> Result<Statement> {
    let value = lower_expression(&local.value).context("Failed to lower value")?;
    Ok(Statement::LocalBinding {
        name: local.name.clone(),
        value,
    })
}

fn lower_config_variable(config: &ConfigVariable) -> Result<Statement> {
    let default = config
        .default_value
        .as_ref()
        .map(lower_expression)
        .transpose()
        .context("Failed to lower config variable default value")?;

    Ok(Statement::ConfigBinding(ConfigBinding {
        name: config.name.clone(),
        config_type: lower_config_type(&config.config_type),
        default,
        secret: config.secret,
    }))
}

fn lower_config_type(ct: &pcl_model::ConfigType) -> ConfigType {
    match ct {
        pcl_model::ConfigType::String => ConfigType::String,
        pcl_model::ConfigType::Number => ConfigType::Number,
        pcl_model::ConfigType::Int => ConfigType::Int,
        pcl_model::ConfigType::Bool => ConfigType::Bool,
        pcl_model::ConfigType::List(inner) => ConfigType::List(Box::new(lower_config_type(inner))),
        pcl_model::ConfigType::Map(inner) => ConfigType::Map(Box::new(lower_config_type(inner))),
    }
}

fn lower_output_variable(output: &OutputVariable) -> Result<Statement> {
    let value = lower_expression(&output.value).context("Failed to lower value")?;
    Ok(Statement::Export {
        name: output.name.clone(),
        value,
    })
}

fn lower_pulumi_block(block: &PulumiBlock) -> Result<Statement> {
    let version_expr = block
        .required_version_range
        .as_ref()
        .context("PulumiBlock requires required_version_range")?;
    let version =
        lower_expression(version_expr).context("Failed to lower required_version_range")?;
    Ok(Statement::RequirePulumiVersion(version))
}

fn lower_resource(resource: &Resource) -> Result<Statement> {
    let token = create_resource_token(resource)
        .context_with(|| format!("Failed to tokenize [{}]", resource.token))?;
    let inputs = resource
        .inputs
        .iter()
        .map(|input| {
            let expr =
                lower_expression(&input.value).context("Failed to lower resource input value")?;
            Ok(ResourceInput {
                name: input.name.clone(),
                expression: expr,
            })
        })
        .collect::<Result<Vec<_>>>()
        .context("Failed to lower resource inputs")?;
    Ok(Statement::Resource {
        name: resource.name.clone(),
        logical_name: resource.logical_name.clone(),
        token,
        inputs,
    })
}

fn create_resource_token(resource: &Resource) -> Result<ResourceToken> {
    if resource.token == "pulumi:index:Stash" {
        Ok(ResourceToken::Stash)
    } else {
        Ok(ResourceToken::Custom {
            provider_name: resource
                .provider_name
                .clone()
                .context_with(|| format!("Resource [{:?}] is missing provider name", resource))?,
            element_id: ElementId::new(&resource.token)
                .into_rootcause()
                .context_with(|| {
                    format!("Failed to convert [{}] into ElementId", resource.token)
                })?,
        })
    }
}

fn lower_expression(expression: &Expression) -> Result<Expr> {
    match &expression.value {
        expression::Value::LiteralValueExpression(lit) => match &lit.value {
            literal_value_expression::Value::UnknownValue(_) => Ok(Expr::Null),
            literal_value_expression::Value::StringValue(s) => Ok(Expr::String(s.clone())),
            literal_value_expression::Value::NumberValue(n) => Ok(Expr::Number(*n)),
            literal_value_expression::Value::BoolValue(b) => Ok(Expr::Bool(*b)),
        },
        expression::Value::TemplateExpression(TemplateExpression { parts }) if parts.len() == 1 => {
            lower_expression(&parts[0])
        }
        expression::Value::TemplateExpression(TemplateExpression { parts }) => {
            let lowered = parts
                .iter()
                .map(lower_expression)
                .collect::<Result<Vec<_>>>()
                .context("Failed to lower template parts")?;
            Ok(Expr::Format { parts: lowered })
        }
        expression::Value::IndexExpression(_) => {
            bail!("IndexExpression not yet supported")
        }
        expression::Value::ObjectConsExpression(obj) => match &expression.expression_type {
            Some(pcl_model::ExpressionType::Dynamic)
            | Some(pcl_model::ExpressionType::Object(_))
            | Some(pcl_model::ExpressionType::None)
            | Some(pcl_model::ExpressionType::Union(_))
            | None => {
                let json = lower_object_to_json(obj, expression)
                    .context("Failed to lower ObjectConsExpression")?;
                Ok(Expr::PulumiAny(json))
            }
            other => bail!(
                "ObjectConsExpression with non-dynamic expression type {:?} is not supported",
                other
            ),
        },
        expression::Value::TupleConsExpression(TupleConsExpression { items }) => {
            let lowered = items
                .iter()
                .map(lower_expression)
                .collect::<Result<Vec<_>>>()
                .context("Failed to lower tuple items")?;
            Ok(Expr::List(lowered))
        }
        expression::Value::FunctionCallExpression(call) => {
            if call.name == "__apply" {
                return Ok(lower_apply_call(call).context("Failed to lower __apply call")?);
            }
            Ok(lower_function_call(call).context("Failed to lower function call")?)
        }
        expression::Value::RelativeTraversalExpression(_) => {
            bail!("RelativeTraversalExpression not yet supported")
        }
        expression::Value::ScopeTraversalExpression(scope) => {
            let mut expr = Expr::Variable(scope.root_name.clone());
            for traverser in &scope.traversal.each {
                match &traverser.value {
                    traverser::Value::TraverseAttr(attr) => {
                        expr = Expr::FieldAccess(Box::new(expr), attr.name.clone());
                    }
                    traverser::Value::TraverseIndex(index) => match &index.value {
                        traverse_index::Value::IntIndex(i) => {
                            expr = Expr::IndexAccess(
                                Box::new(expr),
                                Box::new(Expr::Number(*i as f64)),
                            );
                        }
                        traverse_index::Value::StringIndex(s) => {
                            expr = Expr::IndexAccess(
                                Box::new(expr),
                                Box::new(Expr::String(s.clone())),
                            );
                        }
                    },
                    traverser::Value::TraverseRoot(_) => {
                        // root_name already contains this information
                    }
                    traverser::Value::TraverseSplat(_) => {
                        bail!("TraverseSplat not yet supported")
                    }
                }
            }
            Ok(expr)
        }
        expression::Value::AnonymousFunctionExpression(anon_fn) => {
            let body =
                lower_expression(&anon_fn.body).context("Failed to lower anonymous fn body")?;
            Ok(Expr::Closure {
                params: anon_fn.parameters.clone(),
                body: Box::new(body),
            })
        }
        expression::Value::ConditionalExpression(_) => {
            bail!("ConditionalExpression not yet supported")
        }
        expression::Value::BinaryOpExpression(bin) => {
            let left = lower_expression(&bin.left).context("Failed to lower left operand")?;
            let right = lower_expression(&bin.right).context("Failed to lower right operand")?;
            let op = lower_bin_op(bin.operation)?;
            Ok(Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        expression::Value::UnaryOpExpression(un) => {
            let operand = lower_expression(&un.operand).context("Failed to lower operand")?;
            let op = lower_unary_op(un.operation)?;
            Ok(Expr::UnaryOp {
                op,
                operand: Box::new(operand),
            })
        }
    }
}

fn lower_object_to_json(
    obj: &pcl_model::ObjectConsExpression,
    _parent: &Expression,
) -> Result<JsonValue> {
    let props = obj
        .properties
        .iter()
        .map(|(key, val)| {
            let v = lower_expression_to_json(val)
                .context("Failed to lower ObjectConsExpression property")?;
            Ok((key.clone(), v))
        })
        .collect::<Result<Vec<_>>>()
        .context("Failed to lower ObjectConsExpression properties")?;
    Ok(JsonValue::Object(props))
}

fn lower_expression_to_json(expression: &Expression) -> Result<JsonValue> {
    match &expression.value {
        expression::Value::LiteralValueExpression(lit) => match &lit.value {
            literal_value_expression::Value::UnknownValue(_) => Ok(JsonValue::Null),
            literal_value_expression::Value::StringValue(s) => Ok(JsonValue::String(s.clone())),
            literal_value_expression::Value::NumberValue(n) => Ok(JsonValue::Number(*n)),
            literal_value_expression::Value::BoolValue(b) => Ok(JsonValue::Bool(*b)),
        },
        expression::Value::ObjectConsExpression(obj) => match &expression.expression_type {
            Some(pcl_model::ExpressionType::Dynamic)
            | Some(pcl_model::ExpressionType::Object(_))
            | Some(pcl_model::ExpressionType::None)
            | Some(pcl_model::ExpressionType::Union(_)) => lower_object_to_json(obj, expression),
            other => bail!(
                "ObjectConsExpression with non-dynamic expression type {:?} is not supported",
                other
            ),
        },
        expression::Value::TupleConsExpression(TupleConsExpression { items }) => {
            let elems = items
                .iter()
                .map(lower_expression_to_json)
                .collect::<Result<Vec<_>>>()
                .context("Failed to lower TupleConsExpression items as json values")?;
            Ok(JsonValue::Array(elems))
        }
        _ => {
            let expr =
                lower_expression(expression).context("Failed to lower expression as json value")?;
            Ok(JsonValue::Expr(Box::new(expr)))
        }
    }
}

fn lower_apply_call(call: &FunctionCallExpression) -> Result<Expr> {
    let n_args = call.args.len();
    if n_args < 2 {
        bail!(
            "__apply requires at least 2 arguments (outputs + callback), got {}",
            n_args
        );
    }

    let callback_arg = &call.args[n_args - 1];
    let anon_fn = match &callback_arg.value.value {
        expression::Value::AnonymousFunctionExpression(anon_fn) => anon_fn,
        _ => bail!("Last argument of __apply must be an AnonymousFunctionExpression"),
    };

    let body = lower_expression(&anon_fn.body).context("Failed to lower __apply callback body")?;

    let output_args = &call.args[..n_args - 1];

    if output_args.len() == 1 {
        let output = lower_expression(&output_args[0].value)
            .context("Failed to lower __apply output argument")?;
        let param = anon_fn
            .parameters
            .first()
            .context("__apply callback must have at least one parameter")?;
        Ok(Expr::OutputMap {
            output: Box::new(output),
            params: vec![param.clone()],
            body: Box::new(body),
        })
    } else {
        if output_args.len() > 16 {
            bail!("__apply with more than 16 output arguments is not supported");
        }
        let mut outputs = Vec::new();
        for a in output_args {
            outputs.push(
                lower_expression(&a.value).context("Failed to lower __apply output argument")?,
            );
        }
        Ok(Expr::CombineOutputs {
            outputs,
            params: anon_fn.parameters.clone(),
            body: Box::new(body),
        })
    }
}

fn lower_function_call(call: &FunctionCallExpression) -> Result<Expr> {
    let arg_count = call.args.len();
    let args_lowered = || -> Result<Vec<Expr>> {
        Ok(call
            .args
            .iter()
            .map(|a| lower_expression(&a.value))
            .collect::<Result<Vec<_>>>()
            .context("Failed to lower function call arguments")?)
    };

    match call.name.as_str() {
        "fromBase64" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::FromBase64,
                args: args_lowered()?,
            })
        }
        "toBase64" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::ToBase64,
                args: args_lowered()?,
            })
        }
        "sha1" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Sha1,
                args: args_lowered()?,
            })
        }
        "readFile" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::ReadFile,
                args: args_lowered()?,
            })
        }
        "filebase64" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::FileBase64,
                args: args_lowered()?,
            })
        }
        "filebase64sha256" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::FileBase64Sha256,
                args: args_lowered()?,
            })
        }
        "secret" => {
            ensure_arity(&call.name, arg_count, 1)?;
            let arg =
                lower_expression(&call.args[0].value).context("Failed to lower secret argument")?;
            let arg_type = &call.args[0].r#type;
            if let OutputType(_) = arg_type.value {
                Ok(Expr::MakeSecret(Box::new(arg)))
            } else {
                Ok(Expr::NewSecret(Box::new(arg)))
            }
        }
        "unsecret" => {
            ensure_arity(&call.name, arg_count, 1)?;
            let arg = lower_expression(&call.args[0].value)
                .context("Failed to lower unsecret argument")?;
            let arg_type = &call.args[0].r#type;
            if let OutputType(_) = arg_type.value {
                Ok(Expr::MakeUnsecret(Box::new(arg)))
            } else {
                Ok(Expr::NewOutput(Box::new(arg)))
            }
        }
        "element" => {
            ensure_arity(&call.name, arg_count, 2)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Element,
                args: args_lowered()?,
            })
        }
        "join" => {
            ensure_arity(&call.name, arg_count, 2)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Join,
                args: args_lowered()?,
            })
        }
        "length" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Length,
                args: args_lowered()?,
            })
        }
        "split" => {
            ensure_arity(&call.name, arg_count, 2)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Split,
                args: args_lowered()?,
            })
        }
        "singleOrNone" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::SingleOrNone,
                args: args_lowered()?,
            })
        }
        "cwd" => {
            ensure_arity(&call.name, arg_count, 0)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Cwd,
                args: vec![],
            })
        }
        "rootDirectory" => {
            ensure_arity(&call.name, arg_count, 0)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::RootDirectory,
                args: vec![],
            })
        }
        "stack" => {
            ensure_arity(&call.name, arg_count, 0)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Stack,
                args: vec![],
            })
        }
        "organization" => {
            ensure_arity(&call.name, arg_count, 0)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Organization,
                args: vec![],
            })
        }
        "project" => {
            ensure_arity(&call.name, arg_count, 0)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Project,
                args: vec![],
            })
        }
        "entries" => {
            ensure_arity(&call.name, arg_count, 1)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Entries,
                args: args_lowered()?,
            })
        }
        "lookup" => {
            ensure_arity(&call.name, arg_count, 3)?;
            Ok(Expr::StdlibCall {
                func: StdlibFn::Lookup,
                args: args_lowered()?,
            })
        }
        _ => bail!("Unsupported stdlib function: {}", call.name),
    }
}

fn lower_bin_op(op: pcl_model::Operation) -> Result<BinOp> {
    match op {
        pcl_model::Operation::Add => Ok(BinOp::Add),
        pcl_model::Operation::Subtract => Ok(BinOp::Sub),
        pcl_model::Operation::Multiply => Ok(BinOp::Mul),
        pcl_model::Operation::Divide => Ok(BinOp::Div),
        pcl_model::Operation::Modulo => Ok(BinOp::Mod),
        pcl_model::Operation::Equal => Ok(BinOp::Eq),
        pcl_model::Operation::NotEqual => Ok(BinOp::Ne),
        pcl_model::Operation::GreaterThan => Ok(BinOp::Gt),
        pcl_model::Operation::LessThan => Ok(BinOp::Lt),
        pcl_model::Operation::GreaterThanOrEqual => Ok(BinOp::Ge),
        pcl_model::Operation::LessThanOrEqual => Ok(BinOp::Le),
        pcl_model::Operation::LogicalAnd => Ok(BinOp::And),
        pcl_model::Operation::LogicalOr => Ok(BinOp::Or),
        pcl_model::Operation::LogicalNot | pcl_model::Operation::Negate => {
            bail!("BinaryOpExpression cannot have LogicalNot or Negate operation")
        }
    }
}

fn lower_unary_op(op: pcl_model::Operation) -> Result<UnaryOp> {
    match op {
        pcl_model::Operation::LogicalNot => Ok(UnaryOp::Not),
        pcl_model::Operation::Negate => Ok(UnaryOp::Neg),
        other => bail!("UnaryOpExpression unsupported operation: {:?}", other),
    }
}

fn ensure_arity(name: &str, got: usize, expected: usize) -> Result<()> {
    if got == expected {
        return Ok(());
    }
    bail!(
        "Invalid argument count for function {}: expected {}, got {}",
        name,
        expected,
        got
    )
}
