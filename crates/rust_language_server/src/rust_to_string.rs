/// Renders the Rust IR into source text.
///
/// This layer handles string escaping, formatting, template wrapping,
/// and `prettyplease` formatting. It knows nothing about Pulumi semantics.
use crate::rust_ir::{RustExpr, RustFile, RustStatement};
use rootcause::Result;
use rootcause::prelude::ResultExt;

pub fn render(file: &RustFile) -> Result<String> {
    let statements = file
        .statements
        .iter()
        .map(render_statement)
        .collect::<Vec<_>>()
        .join("\n");

    let source = include_str!("main.rs.template").replace("{{CONTENT}}", &statements);

    let syntax_tree = syn::parse_file(source.as_str())
        .context_with(|| format!("Failed to parse file [{}]", source))?;

    Ok(prettyplease::unparse(&syntax_tree))
}

fn render_statement(stmt: &RustStatement) -> String {
    match stmt {
        RustStatement::Let { name, value } => {
            format!("let {} = {};", name, render_expr(value))
        }
        RustStatement::Expr(expr) => {
            format!("{};", render_expr(expr))
        }
    }
}

pub fn render_expr(expr: &RustExpr) -> String {
    match expr {
        RustExpr::StringLiteral(s) => format!("\"{}\"", s),
        RustExpr::RawStringLiteral(s) => format!("r#\"{}\"#", s),
        RustExpr::NumberLiteral(n) => {
            if *n > (f32::MAX as f64) || *n < (f32::MIN as f64) {
                format!("{}_f64", n)
            } else {
                n.to_string()
            }
        }
        RustExpr::BoolLiteral(b) => b.to_string(),
        RustExpr::Identifier(name) => name.clone(),
        RustExpr::FieldAccess(base, field) => {
            format!("{}.{}", render_expr(base), field)
        }
        RustExpr::IndexAccess(base, index) => {
            format!("{}[{}]", render_expr(base), render_expr(index))
        }
        RustExpr::Vec {
            elements,
            type_hint,
        } => {
            if elements.is_empty() {
                match type_hint {
                    Some(t) => format!("Vec::<{}>::new()", t),
                    None => "Vec::new()".to_string(),
                }
            } else {
                let inner = elements
                    .iter()
                    .map(render_expr)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("vec!({})", inner)
            }
        }
        RustExpr::Format { fmt, args } => {
            if args.is_empty() {
                "String::new()".to_string()
            } else {
                let rendered_args = args.iter().map(render_expr).collect::<Vec<_>>().join(", ");
                format!("format!(\"{}\", {})", fmt, rendered_args)
            }
        }
        RustExpr::FunctionCall { path, args } => {
            let rendered_args = args.iter().map(render_expr).collect::<Vec<_>>().join(", ");
            format!("{}({})", path, rendered_args)
        }
        RustExpr::MethodCall {
            receiver,
            method,
            type_params,
            args,
        } => {
            let rendered_args = args.iter().map(render_expr).collect::<Vec<_>>().join(", ");
            let turbofish = if type_params.is_empty() {
                String::new()
            } else {
                format!("::<{}>", type_params.join(", "))
            };
            format!(
                "{}.{}{}({})",
                render_expr(receiver),
                method,
                turbofish,
                rendered_args
            )
        }
        RustExpr::Closure { params, body } => {
            let params_str = params.join(", ");
            format!("|{}| {}", params_str, render_expr(body))
        }
        RustExpr::BinaryOp { left, op, right } => {
            format!("({} {} {})", render_expr(left), op, render_expr(right))
        }
        RustExpr::UnaryOp { op, operand } => {
            format!("{}{}", op, render_expr(operand))
        }
        RustExpr::MacroCall { path, body } => {
            format!("{}({})", path, body)
        }
        RustExpr::Expect { expr, message } => {
            format!("{}.expect(\"{}\")", render_expr(expr), message)
        }
        RustExpr::Ref(inner) => {
            format!("&{}", render_expr(inner))
        }
        RustExpr::ToStringCall(inner) => {
            format!("({}).to_string()", render_expr(inner))
        }
        RustExpr::Null => "serde_json::Value::Null".to_string(),
    }
}
