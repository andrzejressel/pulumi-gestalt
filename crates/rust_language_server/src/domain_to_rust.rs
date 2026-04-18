/// Lowers the Domain IR into the Rust IR.
///
/// This transform maps Pulumi-semantic concepts (config bindings, output
/// mapping, stdlib calls, etc.) into concrete Rust syntax constructs
/// (let bindings, method calls, function calls, etc.).
use crate::domain_ir::{
    BinOp, ConfigBinding, ConfigType, Expr, JsonValue, Program, Statement, StdlibFn, UnaryOp,
};
use crate::rust_ir::{RustExpr, RustFile, RustStatement};

pub fn lower(program: &Program) -> RustFile {
    RustFile {
        statements: program.statements.iter().map(lower_statement).collect(),
    }
}

fn lower_statement(stmt: &Statement) -> RustStatement {
    match stmt {
        Statement::ConfigBinding(config) => lower_config_binding(config),
        Statement::LocalBinding { name, value } => RustStatement::Let {
            name: name.clone(),
            value: lower_expr(value),
        },
        Statement::Export { name, value } => {
            let lowered = lower_expr(value);
            let arg = match &lowered {
                RustExpr::Vec {
                    elements,
                    type_hint: None,
                } if elements.is_empty() => RustExpr::Ref(Box::new(RustExpr::Vec {
                    elements: vec![],
                    type_hint: Some("String".to_string()),
                })),
                other => RustExpr::Ref(Box::new(other.clone())),
            };
            RustStatement::Expr(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "add_export".to_string(),
                type_params: vec![],
                args: vec![RustExpr::StringLiteral(name.clone()), arg],
            })
        }
        Statement::RequirePulumiVersion(version) => {
            let version_expr = lower_expr(version);
            RustStatement::Expr(RustExpr::Expect {
                expr: Box::new(RustExpr::MethodCall {
                    receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                    method: "require_pulumi_version".to_string(),
                    type_params: vec![],
                    args: vec![RustExpr::Ref(Box::new(version_expr))],
                }),
                message: "Failed to require Pulumi version".to_string(),
            })
        }
    }
}

fn lower_config_binding(config: &ConfigBinding) -> RustStatement {
    let name = &config.name;
    let value = if config.secret {
        lower_config_secret(config)
    } else {
        lower_config_normal(config)
    };
    RustStatement::Let {
        name: name.clone(),
        value,
    }
}

fn lower_config_secret(config: &ConfigBinding) -> RustExpr {
    let name = &config.name;
    match &config.config_type {
        ConfigType::String => RustExpr::Expect {
            expr: Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "require_config_secret".to_string(),
                type_params: vec![],
                args: vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            }),
            message: format!("Expected config [{}] to exist", name),
        },
        ct => RustExpr::Expect {
            expr: Box::new(turbofish_method_call(
                RustExpr::Identifier("ctx".to_string()),
                "require_config_secret_deserialize",
                &rust_config_type(ct),
                vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            )),
            message: format!("Expected config [{}] to exist", name),
        },
    }
}

fn lower_config_normal(config: &ConfigBinding) -> RustExpr {
    let name = &config.name;
    let default_expr = config.default.as_ref().map(lower_expr);

    match (&config.config_type, default_expr) {
        (ConfigType::String, Some(default)) => RustExpr::MethodCall {
            receiver: Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "require_config".to_string(),
                type_params: vec![],
                args: vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            }),
            method: "unwrap_or_else".to_string(),
            type_params: vec![],
            args: vec![RustExpr::Closure {
                params: vec!["_".to_string()],
                body: Box::new(RustExpr::MethodCall {
                    receiver: Box::new(default),
                    method: "to_string".to_string(),
                    type_params: vec![],
                    args: vec![],
                }),
            }],
        },
        (ConfigType::String, None) => RustExpr::Expect {
            expr: Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "require_config".to_string(),
                type_params: vec![],
                args: vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            }),
            message: format!("Expected config [{}] to exist", name),
        },
        (ct, Some(default)) => RustExpr::MethodCall {
            receiver: Box::new(turbofish_method_call(
                RustExpr::Identifier("ctx".to_string()),
                "require_config_deserialize",
                &rust_config_type(ct),
                vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            )),
            method: "unwrap_or".to_string(),
            type_params: vec![],
            args: vec![default],
        },
        (ct, None) => RustExpr::Expect {
            expr: Box::new(turbofish_method_call(
                RustExpr::Identifier("ctx".to_string()),
                "require_config_deserialize",
                &rust_config_type(ct),
                vec![
                    RustExpr::Identifier("None".to_string()),
                    RustExpr::StringLiteral(name.clone()),
                ],
            )),
            message: format!("Expected config [{}] to exist", name),
        },
    }
}

/// Helper to generate `receiver.method::<Type>(args)`.
fn turbofish_method_call(
    receiver: RustExpr,
    method: &str,
    type_param: &str,
    args: Vec<RustExpr>,
) -> RustExpr {
    RustExpr::MethodCall {
        receiver: Box::new(receiver),
        method: method.to_string(),
        type_params: vec![type_param.to_string()],
        args,
    }
}

fn rust_config_type(ct: &ConfigType) -> String {
    match ct {
        ConfigType::String => "String".to_string(),
        ConfigType::Number => "f64".to_string(),
        ConfigType::Int => "i64".to_string(),
        ConfigType::Bool => "bool".to_string(),
        ConfigType::List(inner) => format!("Vec<{}>", rust_config_type(inner)),
        ConfigType::Map(inner) => {
            format!(
                "std::collections::BTreeMap<String, {}>",
                rust_config_type(inner)
            )
        }
    }
}

fn lower_expr(expr: &Expr) -> RustExpr {
    match expr {
        Expr::String(s) => {
            if requires_escaping(s) {
                RustExpr::RawStringLiteral(s.clone())
            } else {
                RustExpr::StringLiteral(s.clone())
            }
        }
        Expr::Number(n) => RustExpr::NumberLiteral(*n),
        Expr::Bool(b) => RustExpr::BoolLiteral(*b),
        Expr::Null => RustExpr::Null,
        Expr::Variable(name) => RustExpr::Identifier(name.clone()),
        Expr::FieldAccess(base, field) => {
            RustExpr::FieldAccess(Box::new(lower_expr(base)), field.clone())
        }
        Expr::IndexAccess(base, index) => {
            let lowered_index = match index.as_ref() {
                Expr::String(s) => RustExpr::StringLiteral(s.clone()),
                Expr::Number(n) => RustExpr::NumberLiteral(*n),
                other => lower_expr(other),
            };
            RustExpr::IndexAccess(Box::new(lower_expr(base)), Box::new(lowered_index))
        }
        Expr::List(items) => {
            let elements = items.iter().map(lower_expr).collect::<Vec<_>>();
            RustExpr::Vec {
                elements,
                type_hint: None,
            }
        }
        Expr::Format { parts } => {
            if parts.is_empty() {
                return RustExpr::FunctionCall {
                    path: "String::new".to_string(),
                    args: vec![],
                };
            }
            let mut fmt = String::new();
            let mut args = Vec::new();
            for part in parts {
                fmt.push_str("{}");
                args.push(lower_expr(part));
            }
            RustExpr::Format { fmt, args }
        }
        Expr::OutputMap {
            output,
            params,
            body,
        } => RustExpr::MethodCall {
            receiver: Box::new(lower_expr(output)),
            method: "map".to_string(),
            type_params: vec![],
            args: vec![RustExpr::Closure {
                params: params.clone(),
                body: Box::new(lower_expr(body)),
            }],
        },
        Expr::CombineOutputs {
            outputs,
            params,
            body,
        } => {
            let n = outputs.len();
            let combine = RustExpr::FunctionCall {
                path: format!("pulumi_gestalt_rust::__private::output::combine{}", n),
                args: outputs.iter().map(lower_expr).collect(),
            };
            RustExpr::MethodCall {
                receiver: Box::new(combine),
                method: "map".to_string(),
                type_params: vec![],
                args: vec![RustExpr::Closure {
                    params: params.clone(),
                    body: Box::new(lower_expr(body)),
                }],
            }
        }
        Expr::MakeSecret(inner) => RustExpr::MethodCall {
            receiver: Box::new(lower_expr(inner)),
            method: "secret".to_string(),
            type_params: vec![],
            args: vec![],
        },
        Expr::MakeUnsecret(inner) => RustExpr::MethodCall {
            receiver: Box::new(lower_expr(inner)),
            method: "unsecret".to_string(),
            type_params: vec![],
            args: vec![],
        },
        Expr::NewSecret(inner) => RustExpr::MethodCall {
            receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
            method: "new_secret".to_string(),
            type_params: vec![],
            args: vec![RustExpr::Ref(Box::new(lower_expr(inner)))],
        },
        Expr::NewOutput(inner) => RustExpr::MethodCall {
            receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
            method: "new_output".to_string(),
            type_params: vec![],
            args: vec![RustExpr::Ref(Box::new(lower_expr(inner)))],
        },
        Expr::PulumiAny(json) => {
            let body = render_json_value(json);
            RustExpr::MacroCall {
                path: "pulumi_gestalt_rust::pulumi_any!".to_string(),
                body,
            }
        }
        Expr::StdlibCall { func, args } => lower_stdlib_call(func, args),
        Expr::BinaryOp { left, op, right } => RustExpr::BinaryOp {
            left: Box::new(lower_expr(left)),
            op: bin_op_str(op),
            right: Box::new(lower_expr(right)),
        },
        Expr::UnaryOp { op, operand } => RustExpr::UnaryOp {
            op: unary_op_str(op),
            operand: Box::new(lower_expr(operand)),
        },
        Expr::Closure { params, body } => RustExpr::Closure {
            params: params.clone(),
            body: Box::new(lower_expr(body)),
        },
    }
}

fn lower_stdlib_call(func: &StdlibFn, args: &[Expr]) -> RustExpr {
    let lowered_args: Vec<RustExpr> = args.iter().map(lower_expr).collect();

    match func {
        StdlibFn::FromBase64 => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::from_base64".to_string(),
                args: lowered_args,
            }),
            message: "Fail to convert from base64".to_string(),
        },
        StdlibFn::ToBase64 => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::to_base64".to_string(),
            args: lowered_args,
        },
        StdlibFn::Sha1 => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::sha1".to_string(),
            args: lowered_args,
        },
        StdlibFn::ReadFile => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::read_file".to_string(),
                args: lowered_args,
            }),
            message: "Failed to read file".to_string(),
        },
        StdlibFn::FileBase64 => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::filebase64".to_string(),
                args: lowered_args,
            }),
            message: "Failed to read file as base64".to_string(),
        },
        StdlibFn::FileBase64Sha256 => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::filebase64sha256".to_string(),
                args: lowered_args,
            }),
            message: "Failed to compute file sha256".to_string(),
        },
        StdlibFn::Element => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::element".to_string(),
                args: vec![
                    RustExpr::Ref(Box::new(lowered_args[0].clone())),
                    lowered_args[1].clone(),
                ],
            }),
            message: "Element should exist".to_string(),
        },
        StdlibFn::Join => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::join".to_string(),
            args: vec![
                lowered_args[0].clone(),
                RustExpr::Ref(Box::new(lowered_args[1].clone())),
            ],
        },
        StdlibFn::Length => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::length".to_string(),
            args: vec![RustExpr::Ref(Box::new(lowered_args[0].clone()))],
        },
        StdlibFn::Split => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::split".to_string(),
            args: lowered_args,
        },
        StdlibFn::SingleOrNone => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::single_or_none".to_string(),
                args: lowered_args,
            }),
            message: "Should get first element".to_string(),
        },
        StdlibFn::Cwd => RustExpr::Expect {
            expr: Box::new(RustExpr::FunctionCall {
                path: "pulumi_gestalt_rust::stdlib::cwd".to_string(),
                args: vec![],
            }),
            message: "Failed to get current directory".to_string(),
        },
        StdlibFn::RootDirectory => {
            RustExpr::ToStringCall(Box::new(RustExpr::Ref(Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "get_root_directory".to_string(),
                type_params: vec![],
                args: vec![],
            }))))
        }
        StdlibFn::Stack => {
            RustExpr::ToStringCall(Box::new(RustExpr::Ref(Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "get_stack".to_string(),
                type_params: vec![],
                args: vec![],
            }))))
        }
        StdlibFn::Organization => {
            RustExpr::ToStringCall(Box::new(RustExpr::Ref(Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "get_organization".to_string(),
                type_params: vec![],
                args: vec![],
            }))))
        }
        StdlibFn::Project => {
            RustExpr::ToStringCall(Box::new(RustExpr::Ref(Box::new(RustExpr::MethodCall {
                receiver: Box::new(RustExpr::Identifier("ctx".to_string())),
                method: "get_project".to_string(),
                type_params: vec![],
                args: vec![],
            }))))
        }
        StdlibFn::Entries => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::entries".to_string(),
            args: vec![RustExpr::Ref(Box::new(lowered_args[0].clone()))],
        },
        StdlibFn::Lookup => RustExpr::FunctionCall {
            path: "pulumi_gestalt_rust::stdlib::lookup".to_string(),
            args: vec![
                RustExpr::Ref(Box::new(lowered_args[0].clone())),
                lowered_args[1].clone(),
                lowered_args[2].clone(),
            ],
        },
    }
}

fn render_json_value(json: &JsonValue) -> String {
    match json {
        JsonValue::String(s) => {
            if requires_escaping(s) {
                format!("r#\"{}\"#", s)
            } else {
                format!("\"{}\"", s)
            }
        }
        JsonValue::Number(n) => {
            if *n > (f32::MAX as f64) || *n < (f32::MIN as f64) {
                format!("{}_f64", n)
            } else {
                n.to_string()
            }
        }
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => "null".to_string(),
        JsonValue::Object(props) => {
            let inner = props
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, render_json_value(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", inner)
        }
        JsonValue::Array(items) => {
            let inner = items
                .iter()
                .map(render_json_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", inner)
        }
        JsonValue::Expr(expr) => {
            // For interpolated expressions we render the Rust IR to string and wrap in parens.
            // This is a slight layering violation but acceptable since `pulumi_any!` is a macro
            // that needs pre-rendered content.
            let rust_expr = lower_expr(expr);
            format!("({})", crate::rust_to_string::render_expr(&rust_expr))
        }
    }
}

fn requires_escaping(s: &str) -> bool {
    s.contains('"') || s.contains('\\') || s.contains('\n') || s.contains('\r') || s.contains('\t')
}

fn bin_op_str(op: &BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Mod => "%",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Gt => ">",
        BinOp::Lt => "<",
        BinOp::Ge => ">=",
        BinOp::Le => "<=",
        BinOp::And => "&&",
        BinOp::Or => "||",
    }
}

fn unary_op_str(op: &UnaryOp) -> &'static str {
    match op {
        UnaryOp::Not => "!",
        UnaryOp::Neg => "-",
    }
}
