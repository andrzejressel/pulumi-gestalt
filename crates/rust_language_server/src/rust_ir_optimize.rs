use crate::rust_ir::RustExpr::Ref;
use crate::rust_ir::{RustExpr, RustFile, RustJsonExpr, RustStatement};

pub fn optimize(file: RustFile) -> RustFile {
    RustFile {
        statements: file
            .statements
            .into_iter()
            .map(optimize_statement)
            .collect(),
    }
}

fn optimize_statement(stmt: RustStatement) -> RustStatement {
    match stmt {
        RustStatement::Let {
            name,
            type_annotation: _,
            value,
        } => {
            let optimized_value = optimize_expr(value);
            let (value, type_annotation) = move_config_type_param_to_let(optimized_value);
            RustStatement::Let {
                name,
                type_annotation,
                value,
            }
        }
        RustStatement::Expr(expr) => RustStatement::Expr(optimize_expr(expr)),
    }
}

fn move_config_type_param_to_let(expr: RustExpr) -> (RustExpr, Option<String>) {
    match expr {
        RustExpr::MethodCall {
            receiver,
            method,
            type_params,
            args,
        } if is_deserialize_method(&method) && type_params.len() == 1 => (
            RustExpr::MethodCall {
                receiver,
                method,
                type_params: vec![],
                args,
            },
            Some(type_params[0].clone()),
        ),
        RustExpr::Expect { expr, message } => match *expr {
            RustExpr::MethodCall {
                receiver,
                method,
                type_params,
                args,
            } if is_deserialize_method(&method) && type_params.len() == 1 => (
                RustExpr::Expect {
                    expr: Box::new(RustExpr::MethodCall {
                        receiver,
                        method,
                        type_params: vec![],
                        args,
                    }),
                    message,
                },
                Some(type_params[0].clone()),
            ),
            inner => (
                RustExpr::Expect {
                    expr: Box::new(inner),
                    message,
                },
                None,
            ),
        },
        other => (other, None),
    }
}

fn is_deserialize_method(method: &str) -> bool {
    method == "require_config_deserialize"
}

fn optimize_expr(expr: RustExpr) -> RustExpr {
    match expr {
        RustExpr::StringLiteral(value) => RustExpr::StringLiteral(value),
        RustExpr::NumberLiteral(value) => RustExpr::NumberLiteral(value),
        RustExpr::BoolLiteral(value) => RustExpr::BoolLiteral(value),
        RustExpr::Identifier(value) => RustExpr::Identifier(value),
        RustExpr::FieldAccess(expr, field) => {
            RustExpr::FieldAccess(Box::new(optimize_expr(*expr)), field)
        }
        RustExpr::IndexAccess(expr, index) => RustExpr::IndexAccess(
            Box::new(optimize_expr(*expr)),
            Box::new(optimize_expr(*index)),
        ),
        RustExpr::Vec {
            elements,
            type_hint,
        } => RustExpr::Vec {
            elements: elements.into_iter().map(optimize_expr).collect(),
            type_hint,
        },
        RustExpr::BTreeMap { entries } => RustExpr::BTreeMap {
            entries: entries
                .into_iter()
                .map(|(key, value)| (optimize_expr(key), optimize_expr(value)))
                .collect(),
        },
        RustExpr::Format { fmt, args } => RustExpr::Format {
            fmt,
            args: args.into_iter().map(optimize_expr).collect(),
        },
        RustExpr::FunctionCall { path, args } => RustExpr::FunctionCall {
            path,
            args: args.into_iter().map(optimize_expr).collect(),
        },
        RustExpr::MethodCall {
            receiver,
            method,
            type_params,
            args,
        } => RustExpr::MethodCall {
            receiver: Box::new(optimize_expr(*receiver)),
            method,
            type_params,
            args: args.into_iter().map(optimize_expr).collect(),
        },
        RustExpr::Closure { params, body } => RustExpr::Closure {
            params,
            body: Box::new(optimize_expr(*body)),
        },
        RustExpr::BinaryOp { left, op, right } => RustExpr::BinaryOp {
            left: Box::new(optimize_expr(*left)),
            op,
            right: Box::new(optimize_expr(*right)),
        },
        RustExpr::UnaryOp { op, operand } => RustExpr::UnaryOp {
            op,
            operand: Box::new(optimize_expr(*operand)),
        },
        RustExpr::PulumiAny(json) => RustExpr::PulumiAny(optimize_json_expr(json)),
        RustExpr::Expect { expr, message } => RustExpr::Expect {
            expr: Box::new(optimize_expr(*expr)),
            message,
        },
        Ref(inner) => Ref(Box::new(optimize_expr(*inner))),
        RustExpr::ToStringCall(inner) => RustExpr::ToStringCall(Box::new(optimize_expr(*inner))),
        RustExpr::Clone(inner) => RustExpr::Clone(Box::new(optimize_expr(*inner))),
        RustExpr::Null => RustExpr::Null,
    }
}

fn optimize_json_expr(expr: RustJsonExpr) -> RustJsonExpr {
    match expr {
        RustJsonExpr::String(value) => RustJsonExpr::String(value),
        RustJsonExpr::Number(value) => RustJsonExpr::Number(value),
        RustJsonExpr::Bool(value) => RustJsonExpr::Bool(value),
        RustJsonExpr::Null => RustJsonExpr::Null,
        RustJsonExpr::Object(entries) => RustJsonExpr::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key, optimize_json_expr(value)))
                .collect(),
        ),
        RustJsonExpr::Array(elements) => {
            RustJsonExpr::Array(elements.into_iter().map(optimize_json_expr).collect())
        }
        RustJsonExpr::Expr(expr) => RustJsonExpr::Expr(Box::new(optimize_expr(*expr))),
    }
}
