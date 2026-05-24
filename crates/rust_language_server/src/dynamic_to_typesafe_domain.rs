/// Lowers the dynamic domain IR into the typesafe domain IR.
///
/// v1 behavior: structural pass-through to establish an explicit boundary
/// between dynamic and typesafe domains in the codegen pipeline.
use crate::dynamic_domain_ir as dynamic;
use crate::typesafe_domain_ir as typesafe;
use rootcause::prelude::ResultExt;
use rootcause::{Result, bail};

pub fn lower(program: &dynamic::Program) -> Result<typesafe::Program> {
    Ok(typesafe::Program {
        statements: program
            .statements
            .iter()
            .map(lower_statement)
            .collect::<Result<Vec<_>>>()
            .context("Failed to lower statements")?,
    })
}

fn lower_statement(statement: &dynamic::Statement) -> Result<typesafe::Statement> {
    Ok(match statement {
        dynamic::Statement::ConfigBinding(config) => {
            typesafe::Statement::ConfigBinding(lower_config_binding(config)?)
        }
        dynamic::Statement::LocalBinding { name, value } => typesafe::Statement::LocalBinding {
            name: name.clone(),
            value: lower_expr(value),
        },
        dynamic::Statement::Export { name, value } => typesafe::Statement::Export {
            name: name.clone(),
            value: lower_expr(value),
        },
        dynamic::Statement::RequirePulumiVersion(version) => {
            typesafe::Statement::RequirePulumiVersion(lower_expr(version))
        }
        dynamic::Statement::Resource {
            name,
            logical_name,
            token,
            inputs,
        } => typesafe::Statement::Resource {
            name: name.clone(),
            logical_name: logical_name.clone(),
            token: lower_resource_token(token),
            inputs: inputs.iter().map(lower_resource_input).collect(),
        },
    })
}

fn lower_resource_input(input: &dynamic::ResourceInput) -> typesafe::ResourceInput {
    typesafe::ResourceInput {
        name: input.name.clone(),
        expression: lower_expr(&input.expression),
    }
}

fn lower_resource_token(token: &dynamic::ResourceToken) -> typesafe::ResourceToken {
    match token {
        dynamic::ResourceToken::Stash => typesafe::ResourceToken::Stash,
        dynamic::ResourceToken::Custom {
            provider_name,
            element_id,
        } => typesafe::ResourceToken::Custom {
            provider_name: provider_name.clone(),
            element_id: element_id.clone(),
        },
    }
}

fn lower_config_binding(binding: &dynamic::ConfigBinding) -> Result<typesafe::ConfigBinding> {
    Ok(typesafe::ConfigBinding {
        name: binding.name.clone(),
        config_type: lower_config_type(&binding.config_type),
        default: binding
            .default
            .as_ref()
            .map(|expr| {
                lower_config_default_expr(expr, &binding.config_type).context_with(|| {
                    format!(
                        "Failed to map config default expression for [{}]",
                        binding.name
                    )
                })
            })
            .transpose()?,
        secret: binding.secret,
    })
}

fn lower_config_default_expr(
    expr: &dynamic::Expr,
    config_type: &dynamic::ConfigType,
) -> Result<typesafe::Expr> {
    map_expr_for_config_type(config_type, expr, "default")
}

fn map_expr_for_config_type(
    config_type: &dynamic::ConfigType,
    expr: &dynamic::Expr,
    path: &str,
) -> Result<typesafe::Expr> {
    match config_type {
        dynamic::ConfigType::String
        | dynamic::ConfigType::Number
        | dynamic::ConfigType::Int
        | dynamic::ConfigType::Bool => map_scalar_expr(config_type, expr, path),
        dynamic::ConfigType::Optional(inner) => lower_optional_config_default(expr, inner, path),
        dynamic::ConfigType::List(inner) => lower_list_config_default(expr, inner, path),
        dynamic::ConfigType::Map(inner) => lower_map_config_default(expr, inner, path),
    }
}

fn map_scalar_expr(
    config_type: &dynamic::ConfigType,
    expr: &dynamic::Expr,
    path: &str,
) -> Result<typesafe::Expr> {
    let expected = lower_config_type_to_expr_type(config_type);
    let converted = convert_expr_for_config_type(config_type, expr, path)?;
    if converted.expr_type != expected {
        bail!(
            "Type mismatch at {path}: expected {:?}, got {:?}",
            expected,
            converted.expr_type
        );
    }
    Ok(converted)
}

fn lower_optional_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
    path: &str,
) -> Result<typesafe::Expr> {
    let lowered_inner_type = lower_config_type_to_expr_type(inner_type);
    let optional_type = typesafe::ExprType::Optional(Box::new(lowered_inner_type));

    if matches!(expr.value, dynamic::ExprValue::Null)
        || matches!(expr.expr_type, dynamic::ExprType::None)
    {
        return Ok(typesafe::Expr {
            expr_type: optional_type,
            value: typesafe::ExprValue::Null,
        });
    }

    let inner = map_expr_for_config_type(inner_type, expr, path)?;
    Ok(typesafe::Expr {
        expr_type: optional_type,
        value: typesafe::ExprValue::Some(Box::new(inner)),
    })
}

fn lower_list_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
    path: &str,
) -> Result<typesafe::Expr> {
    match (&expr.expr_type, &expr.value) {
        (dynamic::ExprType::Tuple(_), dynamic::ExprValue::List(items))
        | (dynamic::ExprType::List(_), dynamic::ExprValue::List(items)) => {
            let lowered_items = items
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    map_expr_for_config_type(inner_type, item, &format!("{path}[{idx}]"))
                })
                .collect::<Result<Vec<_>>>()?;
            let tuple_type = typesafe::ExprType::Tuple(
                lowered_items
                    .iter()
                    .map(|item| item.expr_type.clone())
                    .collect::<Vec<_>>(),
            );
            Ok(typesafe::Expr {
                expr_type: tuple_type,
                value: typesafe::ExprValue::List(lowered_items),
            })
        }
        _ => bail!(
            "Type mismatch at {path}: expected list/tuple expression, got {:?}",
            expr.expr_type
        ),
    }
}

fn lower_map_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
    path: &str,
) -> Result<typesafe::Expr> {
    match &expr.value {
        dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(props))
            if matches!(
                expr.expr_type,
                dynamic::ExprType::Map(_)
                    | dynamic::ExprType::Dynamic
                    | dynamic::ExprType::Object(_)
            ) =>
        {
            Ok(typesafe::Expr {
                expr_type: typesafe::ExprType::Map(Box::new(lower_config_type_to_expr_type(
                    inner_type,
                ))),
                value: typesafe::ExprValue::PulumiAny(typesafe::JsonValue::Object(
                    props
                        .iter()
                        .map(|(k, v)| {
                            Ok((
                                k.clone(),
                                lower_config_default_json(
                                    v,
                                    inner_type,
                                    &format!("{path}[\"{k}\"]"),
                                )?,
                            ))
                        })
                        .collect::<Result<Vec<_>>>()?,
                )),
            })
        }
        _ => bail!(
            "Type mismatch at {path}: expected map/object expression, got {:?}",
            expr.expr_type
        ),
    }
}

fn lower_config_default_json(
    value: &dynamic::JsonValue,
    inner_type: &dynamic::ConfigType,
    path: &str,
) -> Result<typesafe::JsonValue> {
    match value {
        dynamic::JsonValue::Expr(expr) => Ok(typesafe::JsonValue::Expr(Box::new(
            map_expr_for_config_type(inner_type, expr, path)?,
        ))),
        dynamic::JsonValue::Array(items) => match inner_type {
            dynamic::ConfigType::List(item_type) => Ok(typesafe::JsonValue::Array(
                items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        lower_config_default_json(item, item_type, &format!("{path}[{idx}]"))
                    })
                    .collect::<Result<Vec<_>>>()?,
            )),
            _ => bail!(
                "Type mismatch at {path}: expected {:?}, got json array",
                lower_config_type_to_expr_type(inner_type)
            ),
        },
        _ => Ok(lower_json_value(value)),
    }
}

fn convert_expr_for_config_type(
    config_type: &dynamic::ConfigType,
    expr: &dynamic::Expr,
    path: &str,
) -> Result<typesafe::Expr> {
    // Conversion hook for future coercions (e.g. Number -> Int).
    // Current behavior: strict identity conversion only.
    let lowered = lower_expr(expr);
    let expected = lower_config_type_to_expr_type(config_type);
    if lowered.expr_type != expected {
        bail!(
            "No conversion available at {path}: expected {:?}, got {:?}",
            expected,
            lowered.expr_type
        );
    }
    Ok(lowered)
}

fn lower_config_type(config_type: &dynamic::ConfigType) -> typesafe::ConfigType {
    match config_type {
        dynamic::ConfigType::String => typesafe::ConfigType::String,
        dynamic::ConfigType::Number => typesafe::ConfigType::Number,
        dynamic::ConfigType::Int => typesafe::ConfigType::Int,
        dynamic::ConfigType::Bool => typesafe::ConfigType::Bool,
        dynamic::ConfigType::List(inner) => {
            typesafe::ConfigType::List(Box::new(lower_config_type(inner)))
        }
        dynamic::ConfigType::Map(inner) => {
            typesafe::ConfigType::Map(Box::new(lower_config_type(inner)))
        }
        dynamic::ConfigType::Optional(inner) => {
            typesafe::ConfigType::Optional(Box::new(lower_config_type(inner)))
        }
    }
}

fn lower_expr(expr: &dynamic::Expr) -> typesafe::Expr {
    typesafe::Expr {
        expr_type: lower_expr_type(&expr.expr_type),
        value: lower_expr_value(&expr.value),
    }
}

fn lower_expr_value(value: &dynamic::ExprValue) -> typesafe::ExprValue {
    match value {
        dynamic::ExprValue::String(s) => typesafe::ExprValue::String(s.clone()),
        dynamic::ExprValue::Number(n) => typesafe::ExprValue::Number(*n),
        dynamic::ExprValue::Bool(b) => typesafe::ExprValue::Bool(*b),
        dynamic::ExprValue::Null => typesafe::ExprValue::Null,
        dynamic::ExprValue::Variable(name) => typesafe::ExprValue::Variable(name.clone()),
        dynamic::ExprValue::FieldAccess(base, field) => {
            typesafe::ExprValue::FieldAccess(Box::new(lower_expr(base)), field.clone())
        }
        dynamic::ExprValue::IndexAccess(base, index) => typesafe::ExprValue::IndexAccess(
            Box::new(lower_expr(base)),
            Box::new(lower_expr(index)),
        ),
        dynamic::ExprValue::List(items) => {
            typesafe::ExprValue::List(items.iter().map(lower_expr).collect())
        }
        dynamic::ExprValue::Format { parts } => typesafe::ExprValue::Format {
            parts: parts.iter().map(lower_expr).collect(),
        },
        dynamic::ExprValue::OutputMap {
            output,
            params,
            body,
        } => typesafe::ExprValue::OutputMap {
            output: Box::new(lower_expr(output)),
            params: params.clone(),
            body: Box::new(lower_expr(body)),
        },
        dynamic::ExprValue::CombineOutputs {
            outputs,
            params,
            body,
        } => typesafe::ExprValue::CombineOutputs {
            outputs: outputs.iter().map(lower_expr).collect(),
            params: params.clone(),
            body: Box::new(lower_expr(body)),
        },
        dynamic::ExprValue::MakeSecret(inner) => {
            typesafe::ExprValue::MakeSecret(Box::new(lower_expr(inner)))
        }
        dynamic::ExprValue::MakeUnsecret(inner) => {
            typesafe::ExprValue::MakeUnsecret(Box::new(lower_expr(inner)))
        }
        dynamic::ExprValue::NewSecret(inner) => {
            typesafe::ExprValue::NewSecret(Box::new(lower_expr(inner)))
        }
        dynamic::ExprValue::NewOutput(inner) => {
            typesafe::ExprValue::NewOutput(Box::new(lower_expr(inner)))
        }
        dynamic::ExprValue::NewStruct { token, properties } => typesafe::ExprValue::NewStruct {
            token: token.clone(),
            properties: properties
                .iter()
                .map(|(name, expr)| (name.clone(), lower_expr(expr)))
                .collect(),
        },
        dynamic::ExprValue::Map(entries) => typesafe::ExprValue::Map(
            entries
                .iter()
                .map(|(name, expr)| (name.clone(), lower_expr(expr)))
                .collect(),
        ),
        dynamic::ExprValue::PulumiAny(json) => {
            typesafe::ExprValue::PulumiAny(lower_json_value(json))
        }
        dynamic::ExprValue::StdlibCall { func, args } => typesafe::ExprValue::StdlibCall {
            func: lower_stdlib_fn(*func),
            args: args.iter().map(lower_expr).collect(),
        },
        dynamic::ExprValue::BinaryOp { left, op, right } => typesafe::ExprValue::BinaryOp {
            left: Box::new(lower_expr(left)),
            op: lower_bin_op(*op),
            right: Box::new(lower_expr(right)),
        },
        dynamic::ExprValue::UnaryOp { op, operand } => typesafe::ExprValue::UnaryOp {
            op: lower_unary_op(*op),
            operand: Box::new(lower_expr(operand)),
        },
        dynamic::ExprValue::Closure { params, body } => typesafe::ExprValue::Closure {
            params: params.clone(),
            body: Box::new(lower_expr(body)),
        },
    }
}

fn lower_expr_type(expr_type: &dynamic::ExprType) -> typesafe::ExprType {
    match expr_type {
        dynamic::ExprType::String => typesafe::ExprType::String,
        dynamic::ExprType::Number => typesafe::ExprType::Number,
        dynamic::ExprType::Int => typesafe::ExprType::Int,
        dynamic::ExprType::Bool => typesafe::ExprType::Bool,
        dynamic::ExprType::Dynamic => typesafe::ExprType::Dynamic,
        dynamic::ExprType::None => typesafe::ExprType::None,
        dynamic::ExprType::List(inner) => {
            typesafe::ExprType::List(Box::new(lower_expr_type(inner)))
        }
        dynamic::ExprType::Map(inner) => typesafe::ExprType::Map(Box::new(lower_expr_type(inner))),
        dynamic::ExprType::Output(inner) => {
            typesafe::ExprType::Output(Box::new(lower_expr_type(inner)))
        }
        dynamic::ExprType::Tuple(items) => {
            typesafe::ExprType::Tuple(items.iter().map(lower_expr_type).collect())
        }
        dynamic::ExprType::Object(props) => typesafe::ExprType::Object(
            props
                .iter()
                .map(|(k, v)| (k.clone(), lower_expr_type(v)))
                .collect(),
        ),
        dynamic::ExprType::Union(items) => {
            typesafe::ExprType::Union(items.iter().map(lower_expr_type).collect())
        }
    }
}

fn lower_config_type_to_expr_type(config_type: &dynamic::ConfigType) -> typesafe::ExprType {
    match config_type {
        dynamic::ConfigType::String => typesafe::ExprType::String,
        dynamic::ConfigType::Number => typesafe::ExprType::Number,
        dynamic::ConfigType::Int => typesafe::ExprType::Int,
        dynamic::ConfigType::Bool => typesafe::ExprType::Bool,
        dynamic::ConfigType::List(inner) => {
            typesafe::ExprType::List(Box::new(lower_config_type_to_expr_type(inner)))
        }
        dynamic::ConfigType::Map(inner) => {
            typesafe::ExprType::Map(Box::new(lower_config_type_to_expr_type(inner)))
        }
        dynamic::ConfigType::Optional(inner) => {
            typesafe::ExprType::Optional(Box::new(lower_config_type_to_expr_type(inner)))
        }
    }
}

fn lower_json_value(value: &dynamic::JsonValue) -> typesafe::JsonValue {
    match value {
        dynamic::JsonValue::String(s) => typesafe::JsonValue::String(s.clone()),
        dynamic::JsonValue::Number(n) => typesafe::JsonValue::Number(*n),
        dynamic::JsonValue::Bool(b) => typesafe::JsonValue::Bool(*b),
        dynamic::JsonValue::Null => typesafe::JsonValue::Null,
        dynamic::JsonValue::Object(props) => typesafe::JsonValue::Object(
            props
                .iter()
                .map(|(k, v)| (k.clone(), lower_json_value(v)))
                .collect(),
        ),
        dynamic::JsonValue::Array(items) => {
            typesafe::JsonValue::Array(items.iter().map(lower_json_value).collect())
        }
        dynamic::JsonValue::Expr(expr) => typesafe::JsonValue::Expr(Box::new(lower_expr(expr))),
    }
}

fn lower_stdlib_fn(func: dynamic::StdlibFn) -> typesafe::StdlibFn {
    match func {
        dynamic::StdlibFn::FromBase64 => typesafe::StdlibFn::FromBase64,
        dynamic::StdlibFn::ToBase64 => typesafe::StdlibFn::ToBase64,
        dynamic::StdlibFn::Sha1 => typesafe::StdlibFn::Sha1,
        dynamic::StdlibFn::ReadFile => typesafe::StdlibFn::ReadFile,
        dynamic::StdlibFn::FileBase64 => typesafe::StdlibFn::FileBase64,
        dynamic::StdlibFn::FileBase64Sha256 => typesafe::StdlibFn::FileBase64Sha256,
        dynamic::StdlibFn::Element => typesafe::StdlibFn::Element,
        dynamic::StdlibFn::Join => typesafe::StdlibFn::Join,
        dynamic::StdlibFn::Length => typesafe::StdlibFn::Length,
        dynamic::StdlibFn::Split => typesafe::StdlibFn::Split,
        dynamic::StdlibFn::SingleOrNone => typesafe::StdlibFn::SingleOrNone,
        dynamic::StdlibFn::Cwd => typesafe::StdlibFn::Cwd,
        dynamic::StdlibFn::RootDirectory => typesafe::StdlibFn::RootDirectory,
        dynamic::StdlibFn::Stack => typesafe::StdlibFn::Stack,
        dynamic::StdlibFn::Organization => typesafe::StdlibFn::Organization,
        dynamic::StdlibFn::Project => typesafe::StdlibFn::Project,
        dynamic::StdlibFn::Entries => typesafe::StdlibFn::Entries,
        dynamic::StdlibFn::Lookup => typesafe::StdlibFn::Lookup,
        dynamic::StdlibFn::Min => typesafe::StdlibFn::Min,
        dynamic::StdlibFn::Max => typesafe::StdlibFn::Max,
    }
}

fn lower_bin_op(op: dynamic::BinOp) -> typesafe::BinOp {
    match op {
        dynamic::BinOp::Add => typesafe::BinOp::Add,
        dynamic::BinOp::Sub => typesafe::BinOp::Sub,
        dynamic::BinOp::Mul => typesafe::BinOp::Mul,
        dynamic::BinOp::Div => typesafe::BinOp::Div,
        dynamic::BinOp::Mod => typesafe::BinOp::Mod,
        dynamic::BinOp::Eq => typesafe::BinOp::Eq,
        dynamic::BinOp::Ne => typesafe::BinOp::Ne,
        dynamic::BinOp::Gt => typesafe::BinOp::Gt,
        dynamic::BinOp::Lt => typesafe::BinOp::Lt,
        dynamic::BinOp::Ge => typesafe::BinOp::Ge,
        dynamic::BinOp::Le => typesafe::BinOp::Le,
        dynamic::BinOp::And => typesafe::BinOp::And,
        dynamic::BinOp::Or => typesafe::BinOp::Or,
    }
}

fn lower_unary_op(op: dynamic::UnaryOp) -> typesafe::UnaryOp {
    match op {
        dynamic::UnaryOp::Not => typesafe::UnaryOp::Not,
        dynamic::UnaryOp::Neg => typesafe::UnaryOp::Neg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expr(expr_type: dynamic::ExprType, value: dynamic::ExprValue) -> dynamic::Expr {
        dynamic::Expr { expr_type, value }
    }

    fn json_expr(expr: dynamic::Expr) -> dynamic::JsonValue {
        dynamic::JsonValue::Expr(Box::new(expr))
    }

    #[test]
    fn scalar_identity_mapping_succeeds() {
        let mapped = map_expr_for_config_type(
            &dynamic::ConfigType::String,
            &expr(
                dynamic::ExprType::String,
                dynamic::ExprValue::String("abc".to_string()),
            ),
            "default",
        )
        .expect("string config should map from string expr");

        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::String,
                value: typesafe::ExprValue::String("abc".to_string()),
            }
        );
    }

    #[test]
    fn optional_null_maps_to_none() {
        let mapped = map_expr_for_config_type(
            &dynamic::ConfigType::Optional(Box::new(dynamic::ConfigType::String)),
            &expr(dynamic::ExprType::None, dynamic::ExprValue::Null),
            "default",
        )
        .expect("optional null should map to none");

        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::Optional(Box::new(typesafe::ExprType::String)),
                value: typesafe::ExprValue::Null,
            }
        );
    }

    #[test]
    fn optional_non_null_maps_to_some() {
        let mapped = map_expr_for_config_type(
            &dynamic::ConfigType::Optional(Box::new(dynamic::ConfigType::String)),
            &expr(
                dynamic::ExprType::String,
                dynamic::ExprValue::String("abc".to_string()),
            ),
            "default",
        )
        .expect("optional non-null should map to some");

        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::Optional(Box::new(typesafe::ExprType::String)),
                value: typesafe::ExprValue::Some(Box::new(typesafe::Expr {
                    expr_type: typesafe::ExprType::String,
                    value: typesafe::ExprValue::String("abc".to_string()),
                })),
            }
        );
    }

    #[test]
    fn list_accepts_tuple_and_validates_items() {
        let mapped = map_expr_for_config_type(
            &dynamic::ConfigType::List(Box::new(dynamic::ConfigType::String)),
            &expr(
                dynamic::ExprType::Tuple(vec![dynamic::ExprType::String]),
                dynamic::ExprValue::List(vec![expr(
                    dynamic::ExprType::String,
                    dynamic::ExprValue::String("x".to_string()),
                )]),
            ),
            "default",
        )
        .expect("tuple literal should be accepted for list config");

        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::Tuple(vec![typesafe::ExprType::String]),
                value: typesafe::ExprValue::List(vec![typesafe::Expr {
                    expr_type: typesafe::ExprType::String,
                    value: typesafe::ExprValue::String("x".to_string()),
                }]),
            }
        );
    }

    #[test]
    fn map_accepts_dynamic_object_and_validates_values() {
        let mapped = map_expr_for_config_type(
            &dynamic::ConfigType::Map(Box::new(dynamic::ConfigType::Int)),
            &expr(
                dynamic::ExprType::Dynamic,
                dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(vec![(
                    "a".to_string(),
                    json_expr(expr(
                        dynamic::ExprType::Int,
                        dynamic::ExprValue::Number(1.0),
                    )),
                )])),
            ),
            "default",
        )
        .expect("dynamic object should map for map config");

        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::Map(Box::new(typesafe::ExprType::Int)),
                value: typesafe::ExprValue::PulumiAny(typesafe::JsonValue::Object(vec![(
                    "a".to_string(),
                    typesafe::JsonValue::Expr(Box::new(typesafe::Expr {
                        expr_type: typesafe::ExprType::Int,
                        value: typesafe::ExprValue::Number(1.0),
                    })),
                )])),
            }
        );
    }

    #[test]
    fn deeply_nested_mapping_succeeds() {
        let config = dynamic::ConfigType::Map(Box::new(dynamic::ConfigType::List(Box::new(
            dynamic::ConfigType::Map(Box::new(dynamic::ConfigType::Optional(Box::new(
                dynamic::ConfigType::String,
            )))),
        ))));

        let nested = expr(
            dynamic::ExprType::Dynamic,
            dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(vec![(
                "k".to_string(),
                dynamic::JsonValue::Array(vec![dynamic::JsonValue::Expr(Box::new(expr(
                    dynamic::ExprType::Dynamic,
                    dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(vec![
                        ("s".to_string(), dynamic::JsonValue::Null),
                        (
                            "t".to_string(),
                            json_expr(expr(
                                dynamic::ExprType::String,
                                dynamic::ExprValue::String("v".to_string()),
                            )),
                        ),
                    ])),
                )))]),
            )])),
        );

        let mapped = map_expr_for_config_type(&config, &nested, "default")
            .expect("deeply nested map/list/optional should map");
        assert_eq!(
            mapped,
            typesafe::Expr {
                expr_type: typesafe::ExprType::Map(Box::new(typesafe::ExprType::List(Box::new(
                    typesafe::ExprType::Map(Box::new(typesafe::ExprType::Optional(Box::new(
                        typesafe::ExprType::String,
                    )))),
                )))),
                value: typesafe::ExprValue::PulumiAny(typesafe::JsonValue::Object(vec![(
                    "k".to_string(),
                    typesafe::JsonValue::Array(vec![typesafe::JsonValue::Expr(Box::new(
                        typesafe::Expr {
                            expr_type: typesafe::ExprType::Map(Box::new(
                                typesafe::ExprType::Optional(Box::new(typesafe::ExprType::String)),
                            )),
                            value: typesafe::ExprValue::PulumiAny(typesafe::JsonValue::Object(
                                vec![
                                    ("s".to_string(), typesafe::JsonValue::Null),
                                    (
                                        "t".to_string(),
                                        typesafe::JsonValue::Expr(Box::new(typesafe::Expr {
                                            expr_type: typesafe::ExprType::Optional(Box::new(
                                                typesafe::ExprType::String,
                                            )),
                                            value: typesafe::ExprValue::Some(Box::new(
                                                typesafe::Expr {
                                                    expr_type: typesafe::ExprType::String,
                                                    value: typesafe::ExprValue::String(
                                                        "v".to_string(),
                                                    ),
                                                },
                                            )),
                                        })),
                                    ),
                                ],
                            )),
                        },
                    ))]),
                )])),
            }
        );
    }

    #[test]
    fn mismatch_reports_deep_path() {
        let config = dynamic::ConfigType::Map(Box::new(dynamic::ConfigType::List(Box::new(
            dynamic::ConfigType::String,
        ))));
        let invalid = expr(
            dynamic::ExprType::Dynamic,
            dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(vec![(
                "a".to_string(),
                dynamic::JsonValue::Array(vec![json_expr(expr(
                    dynamic::ExprType::Bool,
                    dynamic::ExprValue::Bool(true),
                ))]),
            )])),
        );

        let err = map_expr_for_config_type(&config, &invalid, "default")
            .expect_err("bool inside list<string> should fail");
        let msg = err.to_string();
        assert!(msg.contains("default[\"a\"][0]"));
    }

    #[test]
    fn list_rejects_non_list_shape() {
        let err = map_expr_for_config_type(
            &dynamic::ConfigType::List(Box::new(dynamic::ConfigType::String)),
            &expr(
                dynamic::ExprType::String,
                dynamic::ExprValue::String("nope".to_string()),
            ),
            "default",
        )
        .expect_err("non-list should fail");
        assert!(err.to_string().contains("expected list/tuple expression"));
    }

    #[test]
    fn map_rejects_non_object_shape() {
        let err = map_expr_for_config_type(
            &dynamic::ConfigType::Map(Box::new(dynamic::ConfigType::String)),
            &expr(dynamic::ExprType::Dynamic, dynamic::ExprValue::Null),
            "default",
        )
        .expect_err("non-object map default should fail");
        assert!(err.to_string().contains("expected map/object expression"));
    }
}
