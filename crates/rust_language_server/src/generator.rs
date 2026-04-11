use crate::pcl_model::node::Value;
use crate::pcl_model::{
    ConfigType, ConfigVariable, Expression, LocalVariable, Node, OutputVariable,
    PclProtobufProgram, TemplateExpression, TupleConsExpression, expression,
    literal_value_expression, traverse_index, traverser,
};
use rootcause::prelude::ResultExt;
use rootcause::{Result, bail};

enum ExpressionType {
    EmptyList,
    Other(String),
}

impl ExpressionType {
    fn to_string(&self) -> String {
        match self {
            ExpressionType::EmptyList => "Vec::new()".to_string(),
            ExpressionType::Other(s) => s.clone(),
        }
    }
}

fn requires_escaping(s: &str) -> bool {
    if s.contains('"')
        || s.contains('\\')
        || s.contains('\n')
        || s.contains('\r')
        || s.contains('\t')
    {
        return true;
    }
    false
}

fn escape_rust_string(s: &str) -> String {
    if requires_escaping(s) {
        format!("r#\"{}\"#", s)
    } else {
        format!("\"{}\"", s)
    }
}

pub fn generate_main(model_program: &PclProtobufProgram) -> Result<String> {
    let nodes = model_program
        .nodes
        .iter()
        .map(convert_node)
        .collect::<Result<Vec<_>>>()
        .context("Failed to convert model nodes")?
        .join("\n");

    let file = include_str!("main.rs.template").replace("{{CONTENT}}", &nodes);

    let syntax_tree = syn::parse_file(file.as_str())
        .context_with(|| format!("Failed to parse file [{}]", file))?;

    Ok(prettyplease::unparse(&syntax_tree))
}

fn convert_node(node: &Node) -> Result<String> {
    match &node.value {
        Value::Resource(_) => {
            bail!("Resource not yet supported")
        }
        Value::LocalVariable(local_variable) => {
            Ok(convert_local_variable(local_variable)
                .context("Failed to convert local variable")?)
        }
        Value::ConfigVariable(config_variable) => Ok(convert_config_variable(config_variable)
            .context("Failed to convert config variable")?),
        Value::OutputVariable(output) => {
            Ok(convert_output_variable(output).context("Failed to convert output variable")?)
        }
    }
}

fn convert_config_variable(config_variable: &ConfigVariable) -> Result<String> {
    match &config_variable.config_type {
        ConfigType::String => Ok(format!(
            "let {} = context.require_config(None, \"{}\").expect(\"Expected config [{}] to exist\");",
            config_variable.name, config_variable.name, config_variable.name
        )),
        config_type => Ok(format!(
            "let {} = context.require_config_deserialize::<{}>(None, \"{}\").expect(\"Expected config [{}] to exist\");",
            config_variable.name,
            generate_config_type(config_type),
            config_variable.name,
            config_variable.name
        )),
    }
}

fn generate_config_type(config_type: &ConfigType) -> String {
    match config_type {
        ConfigType::String => "String".to_string(),
        ConfigType::Number => "f64".to_string(),
        ConfigType::Int => "i64".to_string(),
        ConfigType::Bool => "bool".to_string(),
        ConfigType::List(inner) => format!("Vec<{}>", generate_config_type(inner)),
        ConfigType::Map(inner) => format!(
            "std::collections::BTreeMap<String, {}>",
            generate_config_type(inner)
        ),
    }
}

fn convert_local_variable(local_variable: &LocalVariable) -> Result<String> {
    let value = convert_expression(&local_variable.value).context("Failed to convert value")?;
    Ok(format!(
        "let {} = {};",
        local_variable.name,
        value.to_string()
    ))
}

fn convert_output_variable(output_variable: &OutputVariable) -> Result<String> {
    let value = convert_expression(&output_variable.value).context("Failed to convert value")?;
    match value {
        ExpressionType::EmptyList => Ok(format!(
            "context.add_export(\"{}\", &Vec::<String>::new());",
            output_variable.name
        )),
        ExpressionType::Other(s) => Ok(format!(
            "context.add_export(\"{}\", &{});",
            output_variable.name, s
        )),
    }
}

fn convert_expression(expression: &Expression) -> Result<ExpressionType> {
    match &expression.value {
        expression::Value::LiteralValueExpression(literal_value) => match &literal_value.value {
            literal_value_expression::Value::UnknownValue(_) => {
                bail!("UnknownValue not yet supported")
            }
            literal_value_expression::Value::StringValue(s) => {
                Ok(ExpressionType::Other(escape_rust_string(s)))
            }
            literal_value_expression::Value::NumberValue(n) => {
                Ok(ExpressionType::Other(n.to_string()))
            }
            literal_value_expression::Value::BoolValue(b) => {
                Ok(ExpressionType::Other(b.to_string()))
            }
        },
        expression::Value::TemplateExpression(TemplateExpression { parts }) if parts.len() == 1 => {
            let el = &parts[0];
            convert_expression(el)
        }
        expression::Value::TemplateExpression(expr) => {
            bail!("TemplateExpression not yet supported [{expr:?}]")
        }
        expression::Value::IndexExpression(_) => {
            bail!("IndexExpression not yet supported")
        }
        expression::Value::ObjectConsExpression(_) => {
            bail!("ObjectConsExpression not yet supported")
        }
        expression::Value::TupleConsExpression(TupleConsExpression { items }) => {
            let converted_items = items
                .iter()
                .map(convert_expression)
                .collect::<Result<Vec<_>>>()
                .context("Failed to convert tuple items")?
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if items.is_empty() {
                Ok(ExpressionType::EmptyList)
            } else {
                Ok(ExpressionType::Other(format!("vec!({})", converted_items)))
            }
        }
        expression::Value::FunctionCallExpression(function_call) => {
            let args = function_call
                .args
                .iter()
                .map(|a| &a.value)
                .map(convert_expression)
                .collect::<Result<Vec<_>>>()
                .context("Failed to convert function call arguments")?
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Ok(ExpressionType::Other(
                convert_stdlib_function_call(
                    &function_call.name,
                    args,
                    function_call.args.iter().map(|a| &a.value).collect(),
                    function_call.args.len(),
                )
                .context("Failed to convert function call")?,
            ))
        }
        expression::Value::RelativeTraversalExpression(_) => {
            bail!("RelativeTraversalExpression not yet supported")
        }
        expression::Value::ScopeTraversalExpression(scope_traversal) => {
            let mut converted = scope_traversal.root_name.clone();
            for traverser in &scope_traversal.traversal.each {
                match &traverser.value {
                    traverser::Value::TraverseAttr(attr) => {
                        converted = format!("{}.{}", converted, attr.name);
                    }
                    traverser::Value::TraverseIndex(index) => match &index.value {
                        traverse_index::Value::IntIndex(i) => {
                            converted = format!("{}[{}]", converted, i)
                        }
                        traverse_index::Value::StringIndex(s) => {
                            converted = format!("{}[\"{}\"]", converted, s)
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
            Ok(ExpressionType::Other(converted))
        }
        expression::Value::AnonymousFunctionExpression(_) => {
            bail!("AnonymousFunctionExpression not yet supported")
        }
        expression::Value::ConditionalExpression(_) => {
            bail!("ConditionalExpression not yet supported")
        }
        expression::Value::BinaryOpExpression(_) => {
            bail!("BinaryOpExpression not yet supported")
        }
        expression::Value::UnaryOpExpression(_) => {
            bail!("UnaryOpExpression not yet supported")
        }
    }
}

fn convert_stdlib_function_call(
    name: &str,
    args: String,
    args_pure: Vec<&Expression>,
    arg_count: usize,
) -> Result<String> {
    match name {
        "fromBase64" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::from_base64({}).expect(\"Fail to convert from base64\")",
                args
            ))
        }
        "toBase64" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!("pulumi_gestalt_rust::stdlib::to_base64({})", args))
        }
        "sha1" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!("pulumi_gestalt_rust::stdlib::sha1({})", args))
        }
        "secret" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "{}.as_secret_output(context)",
                args
            ))
        }
        "unsecret" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "{}.as_unsecret_output(context)",
                args
            ))
        }
        "element" => {
            ensure_arity(name, arg_count, 2)?;
            let first_arg = convert_expression(args_pure[0])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(args_pure[1])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::element(&{}, {}).expect(\"Element should exist\")",
                first_arg, second_arg
            ))
        }
        "join" => {
            ensure_arity(name, arg_count, 2)?;
            let first_arg = convert_expression(args_pure[0])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(args_pure[1])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::join({}, &{})",
                first_arg, second_arg
            ))
        }
        "length" => {
            ensure_arity(name, arg_count, 1)?;
            let first_arg = convert_expression(args_pure[0])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::length(&{})",
                first_arg
            ))
        }
        "split" => {
            ensure_arity(name, arg_count, 2)?;
            Ok(format!("pulumi_gestalt_rust::stdlib::split({})", args))
        }
        "singleOrNone" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::single_or_none({}).expect(\"Should get first element\")",
                args
            ))
        }
        "cwd" => {
            ensure_arity(name, arg_count, 0)?;
            Ok(
                "pulumi_gestalt_rust::stdlib::cwd().expect(\"Failed to get current directory\")"
                    .to_string(),
            )
        }
        "rootDirectory" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&context.get_root_directory()).to_string()".to_string())
        }
        "stack" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&context.get_stack()).to_string()".to_string())
        }
        "organization" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&context.get_organization()).to_string()".to_string())
        }
        "project" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&context.get_project()).to_string()".to_string())
        }
        "entries" => {
            ensure_arity(name, arg_count, 1)?;
            let first_arg = convert_expression(args_pure[0])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::entries(&{first_arg})"
            ))
        }
        "lookup" => {
            ensure_arity(name, arg_count, 3)?;
            let first_arg = convert_expression(args_pure[0])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(args_pure[1])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            let third_arg = convert_expression(args_pure[2])
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[2]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::lookup(&{}, {}, {})",
                first_arg, second_arg, third_arg
            ))
        }
        _ => bail!("Unsupported stdlib function: {}", name),
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
