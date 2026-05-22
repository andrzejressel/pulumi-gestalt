/// Lowers the dynamic domain IR into the typesafe domain IR.
///
/// v1 behavior: structural pass-through to establish an explicit boundary
/// between dynamic and typesafe domains in the codegen pipeline.
use crate::dynamic_domain_ir as dynamic;
use crate::typesafe_domain_ir as typesafe;

pub fn lower(program: &dynamic::Program) -> rootcause::Result<typesafe::Program> {
    Ok(typesafe::Program {
        statements: program.statements.iter().map(lower_statement).collect(),
    })
}

fn lower_statement(statement: &dynamic::Statement) -> typesafe::Statement {
    match statement {
        dynamic::Statement::ConfigBinding(config) => {
            typesafe::Statement::ConfigBinding(lower_config_binding(config))
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
    }
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

fn lower_config_binding(binding: &dynamic::ConfigBinding) -> typesafe::ConfigBinding {
    typesafe::ConfigBinding {
        name: binding.name.clone(),
        config_type: lower_config_type(&binding.config_type),
        default: binding
            .default
            .as_ref()
            .map(|expr| lower_config_default_expr(expr, &binding.config_type)),
        secret: binding.secret,
    }
}

fn lower_config_default_expr(
    expr: &dynamic::Expr,
    config_type: &dynamic::ConfigType,
) -> typesafe::Expr {
    match config_type {
        dynamic::ConfigType::Optional(inner) => lower_optional_config_default(expr, inner),
        dynamic::ConfigType::List(inner) => lower_list_config_default(expr, inner),
        dynamic::ConfigType::Map(inner) => lower_map_config_default(expr, inner),
        _ => lower_expr(expr),
    }
}

fn lower_optional_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
) -> typesafe::Expr {
    let lowered_inner_type = lower_config_type_to_expr_type(inner_type);
    let optional_type = typesafe::ExprType::Optional(Box::new(lowered_inner_type));

    if matches!(expr.value, dynamic::ExprValue::Null)
        || matches!(expr.expr_type, dynamic::ExprType::None)
    {
        return typesafe::Expr {
            expr_type: optional_type,
            value: typesafe::ExprValue::Null,
        };
    }

    let inner = lower_config_default_expr(expr, inner_type);
    typesafe::Expr {
        expr_type: optional_type,
        value: typesafe::ExprValue::Some(Box::new(inner)),
    }
}

fn lower_list_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
) -> typesafe::Expr {
    match (&expr.expr_type, &expr.value) {
        (dynamic::ExprType::Tuple(_), dynamic::ExprValue::List(items))
        | (dynamic::ExprType::List(_), dynamic::ExprValue::List(items)) => {
            let lowered_items = items
                .iter()
                .map(|item| lower_config_default_expr(item, inner_type))
                .collect::<Vec<_>>();
            let tuple_type = typesafe::ExprType::Tuple(
                lowered_items
                    .iter()
                    .map(|item| item.expr_type.clone())
                    .collect::<Vec<_>>(),
            );
            typesafe::Expr {
                expr_type: tuple_type,
                value: typesafe::ExprValue::List(lowered_items),
            }
        }
        _ => lower_expr(expr),
    }
}

fn lower_map_config_default(
    expr: &dynamic::Expr,
    inner_type: &dynamic::ConfigType,
) -> typesafe::Expr {
    match &expr.value {
        dynamic::ExprValue::PulumiAny(dynamic::JsonValue::Object(props)) => typesafe::Expr {
            expr_type: typesafe::ExprType::Map(Box::new(lower_config_type_to_expr_type(
                inner_type,
            ))),
            value: typesafe::ExprValue::PulumiAny(typesafe::JsonValue::Object(
                props
                    .iter()
                    .map(|(k, v)| (k.clone(), lower_config_default_json(v, inner_type)))
                    .collect(),
            )),
        },
        _ => lower_expr(expr),
    }
}

fn lower_config_default_json(
    value: &dynamic::JsonValue,
    inner_type: &dynamic::ConfigType,
) -> typesafe::JsonValue {
    match value {
        dynamic::JsonValue::Expr(expr) => {
            typesafe::JsonValue::Expr(Box::new(lower_config_default_expr(expr, inner_type)))
        }
        _ => lower_json_value(value),
    }
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
