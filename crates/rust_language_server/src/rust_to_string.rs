/// Renders the Rust IR into source text.
///
/// This layer handles string escaping, formatting, template wrapping,
/// and `prettyplease` formatting. It knows nothing about Pulumi semantics.
use crate::rust_ir::{RustExpr, RustFile, RustJsonExpr, RustStatement};
use quote::quote;
use rootcause::Result;

pub fn render(file: &RustFile, packages_expr: &str) -> Result<String> {
    let statements = file
        .statements
        .iter()
        .map(render_statement)
        .collect::<Vec<_>>()
        .join("\n");

    let source = include_str!("main.rs.template")
        .replace("{{CONTENT}}", &statements)
        .replace("{{PACKAGES}}", packages_expr);

    let syntax_tree = syn::parse_file(source.as_str());

    match syntax_tree {
        Ok(syntax_tree) => Ok(prettyplease::unparse(&syntax_tree)),
        Err(_) => {
            // It will not compile anyway, but at least we will have a file to debug
            Ok(source)
        }
    }
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
        RustExpr::StringLiteral(s) => quote! { #s }.to_string(),
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
        RustExpr::BTreeMap { entries } => {
            if entries.is_empty() {
                "std::collections::BTreeMap::new()".to_string()
            } else {
                let inner = entries
                    .iter()
                    // FIXME
                    .map(|(k, v)| {
                        format!(
                            "(({}).to_string(), ({}).to_string())",
                            render_expr(k),
                            render_expr(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("std::collections::BTreeMap::from([{}])", inner)
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
        RustExpr::PulumiAny(value) => {
            format!(
                "pulumi_gestalt_rust::pulumi_any!({})",
                render_json_expr(value)
            )
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
        RustExpr::Clone(inner) => {
            format!("({}).clone()", render_expr(inner))
        }
        RustExpr::Null => "pulumi_gestalt_rust::pulumi_any!(null)".to_string(),
    }
}

fn render_json_expr(expr: &RustJsonExpr) -> String {
    match expr {
        RustJsonExpr::String(s) => {
            let lit = LitStr::new(s, proc_macro2::Span::call_site());
            quote! { #lit }.to_string()
        }
        RustJsonExpr::Number(n) => {
            if *n > (f32::MAX as f64) || *n < (f32::MIN as f64) {
                format!("{}_f64", n)
            } else {
                n.to_string()
            }
        }
        RustJsonExpr::Bool(b) => b.to_string(),
        RustJsonExpr::Null => "null".to_string(),
        RustJsonExpr::Object(props) => {
            let inner = props
                .iter()
                .map(|(k, v)| {
                    let key = LitStr::new(k, proc_macro2::Span::call_site());
                    format!("{}: {}", quote! { #key }, render_json_expr(v))
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{inner}}}")
        }
        RustJsonExpr::Array(items) => {
            let inner = items
                .iter()
                .map(render_json_expr)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{inner}]")
        }
        RustJsonExpr::Expr(expr) => format!("({})", render_expr(expr)),
    }
}
