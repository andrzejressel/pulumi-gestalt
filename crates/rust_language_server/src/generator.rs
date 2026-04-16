use crate::pcl_model::node::Value;
use crate::pcl_model::r#type::Value::OutputType;
use crate::pcl_model::{
    ConfigType, ConfigVariable, Expression, FunctionCallArgument, FunctionCallExpression,
    LocalVariable, Node, Operation, OutputVariable, PclProtobufProgram, PulumiBlock,
    TemplateExpression, TupleConsExpression, expression, literal_value_expression, traverse_index,
    traverser,
};
use rootcause::option_ext::OptionExt;
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
        Value::PulumiBlock(pulumi_block) => {
            Ok(convert_pulumi_block(pulumi_block).context("Failed to convert pulumi block")?)
        }
    }
}

fn convert_config_variable(config_variable: &ConfigVariable) -> Result<String> {
    let default_expr = match &config_variable.default_value {
        Some(expr) => Some(
            convert_expression(expr)
                .context("Failed to convert config variable default value")?
                .to_string(),
        ),
        None => None,
    };

    if config_variable.secret {
        match &config_variable.config_type {
            ConfigType::String => Ok(format!(
                "let {} = ctx.require_config_secret(None, \"{}\").expect(\"Expected config [{}] to exist\");",
                config_variable.name, config_variable.name, config_variable.name
            )),
            config_type => Ok(format!(
                "let {} = ctx.require_config_secret_deserialize::<{}>(None, \"{}\").expect(\"Expected config [{}] to exist\");",
                config_variable.name,
                generate_config_type(config_type),
                config_variable.name,
                config_variable.name
            )),
        }
    } else {
        match (&config_variable.config_type, default_expr) {
            (ConfigType::String, Some(default)) => Ok(format!(
                "let {} = ctx.require_config(None, \"{}\").unwrap_or_else(|_| {}.to_string());",
                config_variable.name, config_variable.name, default
            )),
            (ConfigType::String, None) => Ok(format!(
                "let {} = ctx.require_config(None, \"{}\").expect(\"Expected config [{}] to exist\");",
                config_variable.name, config_variable.name, config_variable.name
            )),
            (config_type, Some(default)) => Ok(format!(
                "let {} = ctx.require_config_deserialize::<{}>(None, \"{}\").unwrap_or({});",
                config_variable.name,
                generate_config_type(config_type),
                config_variable.name,
                default
            )),
            (config_type, None) => Ok(format!(
                "let {} = ctx.require_config_deserialize::<{}>(None, \"{}\").expect(\"Expected config [{}] to exist\");",
                config_variable.name,
                generate_config_type(config_type),
                config_variable.name,
                config_variable.name
            )),
        }
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
            "ctx.add_export(\"{}\", &Vec::<String>::new());",
            output_variable.name
        )),
        ExpressionType::Other(s) => Ok(format!(
            "ctx.add_export(\"{}\", &{});",
            output_variable.name, s
        )),
    }
}

fn convert_pulumi_block(pulumi_block: &PulumiBlock) -> Result<String> {
    let version_expr = match &pulumi_block.required_version_range {
        Some(expr) => expr,
        None => bail!("PulumiBlock requires required_version_range"),
    };
    let version = convert_expression(version_expr)
        .context("Failed to convert required_version_range")?
        .to_string();
    Ok(format!(
        "ctx.require_pulumi_version(&{version}).expect(\"Failed to require Pulumi version\");"
    ))
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
                if n > &(f32::MAX as f64) || n < &(f32::MIN as f64) {
                    Ok(ExpressionType::Other(format!("{}_f64", n)))
                } else {
                    Ok(ExpressionType::Other(n.to_string()))
                }
            }
            literal_value_expression::Value::BoolValue(b) => {
                Ok(ExpressionType::Other(b.to_string()))
            }
        },
        expression::Value::TemplateExpression(TemplateExpression { parts }) if parts.len() == 1 => {
            let el = &parts[0];
            convert_expression(el)
        }
        expression::Value::TemplateExpression(TemplateExpression { parts }) => {
            let mut format_string = String::new();
            let mut args = Vec::new();

            for part in parts {
                let converted =
                    convert_expression(part).context("Failed to convert template part")?;
                match converted {
                    ExpressionType::Other(s) => {
                        format_string.push_str("{}");
                        args.push(s);
                    }
                    ExpressionType::EmptyList => {
                        format_string.push_str("{:?}");
                        args.push("Vec::new()".to_string());
                    }
                }
            }

            if args.is_empty() {
                Ok(ExpressionType::Other("String::new()".to_string()))
            } else {
                Ok(ExpressionType::Other(format!(
                    "format!(\"{}\", {})",
                    format_string,
                    args.join(", ")
                )))
            }
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
            // The __apply intrinsic is produced by pcl.RewriteApplies and requires special
            // handling: the last arg is an AnonymousFunctionExpression (the callback), and the
            // preceding args are the Output-typed expressions to apply over.
            if function_call.name == "__apply" {
                return Ok(ExpressionType::Other(
                    convert_apply_call(function_call).context("Failed to convert __apply call")?,
                ));
            }
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
                convert_stdlib_function_call(&function_call.name, args, &function_call.args)
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
        expression::Value::AnonymousFunctionExpression(anon_fn) => {
            let params = anon_fn.parameters.join(", ");
            let body = convert_expression(&anon_fn.body)
                .context("Failed to convert anonymous function body")?
                .to_string();
            Ok(ExpressionType::Other(format!("|{}| {}", params, body)))
        }
        expression::Value::ConditionalExpression(_) => {
            bail!("ConditionalExpression not yet supported")
        }
        expression::Value::BinaryOpExpression(binary_op) => {
            let left =
                convert_expression(&binary_op.left).context("Failed to convert left operand")?;
            let right =
                convert_expression(&binary_op.right).context("Failed to convert right operand")?;

            let left_str = left.to_string();
            let right_str = right.to_string();

            let op_str = match binary_op.operation {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "*",
                Operation::Divide => "/",
                Operation::Modulo => "%",
                Operation::Equal => "==",
                Operation::NotEqual => "!=",
                Operation::GreaterThan => ">",
                Operation::LessThan => "<",
                Operation::GreaterThanOrEqual => ">=",
                Operation::LessThanOrEqual => "<=",
                Operation::LogicalAnd => "&&",
                Operation::LogicalOr => "||",
                Operation::LogicalNot | Operation::Negate => {
                    bail!("BinaryOpExpression cannot have LogicalNot or Negate operation")
                }
            };

            Ok(ExpressionType::Other(format!(
                "({} {} {})",
                left_str, op_str, right_str
            )))
        }
        expression::Value::UnaryOpExpression(unary_op) => {
            let operand =
                convert_expression(&unary_op.operand).context("Failed to convert operand")?;
            let operand_str = operand.to_string();

            let result = match unary_op.operation {
                Operation::LogicalNot => format!("!{}", operand_str),
                Operation::Negate => format!("-{}", operand_str),
                op => bail!("UnaryOpExpression unsupported operation: {:?}", op),
            };

            Ok(ExpressionType::Other(result))
        }
    }
}

/// Generates Rust code for a `__apply` intrinsic call produced by `pcl.RewriteApplies`.
///
/// The `__apply` call has the shape:
///   `__apply(output_arg_1, ..., output_arg_N, anonymous_fn)`
///
/// For a single output argument this emits:
///   `output_arg.map(|param| body)`
///
/// For N > 1 output arguments this emits:
///   `pulumi_gestalt_rust::__private::output::combineN(o1, o2, ...).map(|(p1, p2, ...)| body)`
fn convert_apply_call(function_call: &FunctionCallExpression) -> Result<String> {
    let n_args = function_call.args.len();
    if n_args < 2 {
        bail!(
            "__apply requires at least 2 arguments (outputs + callback), got {}",
            n_args
        );
    }

    // Last arg must be the AnonymousFunctionExpression (the callback).
    let callback_arg = &function_call.args[n_args - 1];
    let anon_fn = match &callback_arg.value.value {
        expression::Value::AnonymousFunctionExpression(anon_fn) => anon_fn,
        _ => bail!("Last argument of __apply must be an AnonymousFunctionExpression"),
    };

    let body = convert_expression(&anon_fn.body)
        .context("Failed to convert __apply callback body")?
        .to_string();

    // All args except the last are the Output-typed expressions to apply over.
    let output_args = &function_call.args[..n_args - 1];

    if output_args.len() == 1 {
        // Single output: output.map(|param| body)
        let output_expr = convert_expression(&output_args[0].value)
            .context("Failed to convert __apply output argument")?
            .to_string();
        let param = anon_fn
            .parameters
            .first()
            .context("__apply callback must have at least one parameter")?;
        Ok(format!("{}.map(|{}| {})", output_expr, param, body))
    } else {
        // Multiple outputs: combineN(o1, o2, ...).map(|(p1, p2, ...)| body)
        let n = output_args.len();
        if n > 16 {
            bail!("__apply with more than 16 output arguments is not supported");
        }
        let mut output_exprs = Vec::new();
        for a in output_args {
            output_exprs.push(
                convert_expression(&a.value)
                    .context("Failed to convert __apply output argument")?
                    .to_string(),
            );
        }
        let combine_fn = format!("pulumi_gestalt_rust::__private::output::combine{}", n);
        let combine_args = output_exprs.join(", ");
        let params = anon_fn.parameters.join(", ");
        Ok(format!(
            "{}({}).map(|({})| {})",
            combine_fn, combine_args, params, body
        ))
    }
}

fn convert_stdlib_function_call(
    name: &str,
    args: String,
    args_pure: &[FunctionCallArgument],
) -> Result<String> {
    let arg_count = args_pure.len();
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
        "readFile" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::read_file({}).expect(\"Failed to read file\")",
                args
            ))
        }
        "filebase64" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::filebase64({}).expect(\"Failed to read file as base64\")",
                args
            ))
        }
        "filebase64sha256" => {
            ensure_arity(name, arg_count, 1)?;
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::filebase64sha256({}).expect(\"Failed to compute file sha256\")",
                args
            ))
        }
        "secret" => {
            ensure_arity(name, arg_count, 1)?;
            let arg_type = &args_pure[0].r#type;
            if let OutputType(_) = arg_type.value {
                Ok(format!("{}.secret()", args))
            } else {
                Ok(format!("ctx.new_secret(&{})", args))
            }
        }
        "unsecret" => {
            ensure_arity(name, arg_count, 1)?;
            let arg_type = &args_pure[0].r#type;
            if let OutputType(_) = arg_type.value {
                Ok(format!("{}.unsecret()", args))
            } else {
                Ok(format!("ctx.new_output(&{})", args))
            }
        }
        "element" => {
            ensure_arity(name, arg_count, 2)?;
            let first_arg = convert_expression(&args_pure[0].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(&args_pure[1].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::element(&{}, {}).expect(\"Element should exist\")",
                first_arg, second_arg
            ))
        }
        "join" => {
            ensure_arity(name, arg_count, 2)?;
            let first_arg = convert_expression(&args_pure[0].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(&args_pure[1].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::join({}, &{})",
                first_arg, second_arg
            ))
        }
        "length" => {
            ensure_arity(name, arg_count, 1)?;
            let first_arg = convert_expression(&args_pure[0].value)
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
            Ok("(&ctx.get_root_directory()).to_string()".to_string())
        }
        "stack" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&ctx.get_stack()).to_string()".to_string())
        }
        "organization" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&ctx.get_organization()).to_string()".to_string())
        }
        "project" => {
            ensure_arity(name, arg_count, 0)?;
            Ok("(&ctx.get_project()).to_string()".to_string())
        }
        "entries" => {
            ensure_arity(name, arg_count, 1)?;
            let first_arg = convert_expression(&args_pure[0].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            Ok(format!(
                "pulumi_gestalt_rust::stdlib::entries(&{first_arg})"
            ))
        }
        "lookup" => {
            ensure_arity(name, arg_count, 3)?;
            let first_arg = convert_expression(&args_pure[0].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[0]))?
                .to_string();
            let second_arg = convert_expression(&args_pure[1].value)
                .context_with(|| format!("Failed to convert argument [{:?}]", args_pure[1]))?
                .to_string();
            let third_arg = convert_expression(&args_pure[2].value)
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
