/// Rust IR: target-language syntax tree.
///
/// This layer represents Rust source code constructs with no knowledge of
/// Pulumi semantics. It is produced by lowering the Domain IR and consumed
/// by the renderer that emits source text.

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct RustFile {
    pub statements: Vec<RustStatement>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum RustStatement {
    Let { name: String, value: RustExpr },
    Expr(RustExpr),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum RustExpr {
    /// `"hello"` — regular string literal.
    StringLiteral(String),
    /// `r#"hello"#` — raw string literal (no escape processing).
    RawStringLiteral(String),
    /// Numeric literal. Emitted as `{value}` or `{value}_f64` for large values.
    NumberLiteral(f64),
    /// `true` / `false`.
    BoolLiteral(bool),
    /// A bare identifier (variable name, path segment, etc.).
    Identifier(String),
    /// `expr.field`
    FieldAccess(Box<RustExpr>, String),
    /// `expr[index]` with string or integer key.
    IndexAccess(Box<RustExpr>, Box<RustExpr>),
    /// `vec![a, b, c]` or `Vec::<T>::new()` when empty with a type hint.
    Vec {
        elements: Vec<RustExpr>,
        type_hint: Option<String>,
    },
    /// `format!("...", args...)`
    Format { fmt: String, args: Vec<RustExpr> },
    /// `path::to::func(args...)`
    FunctionCall { path: String, args: Vec<RustExpr> },
    /// `receiver.method(args...)` or `receiver.method::<T>(args...)` with type parameters
    MethodCall {
        receiver: Box<RustExpr>,
        method: String,
        type_params: Vec<String>,
        args: Vec<RustExpr>,
    },
    /// `|params| body`
    Closure {
        params: Vec<String>,
        body: Box<RustExpr>,
    },
    /// `left op right` — operator stored as the literal token (e.g. `"+"`).
    BinaryOp {
        left: Box<RustExpr>,
        op: &'static str,
        right: Box<RustExpr>,
    },
    /// `op operand` — prefix unary operator (e.g. `"!"`, `"-"`).
    UnaryOp {
        op: &'static str,
        operand: Box<RustExpr>,
    },
    /// `path!(body)` — macro invocation with a pre-rendered body.
    MacroCall { path: String, body: String },
    /// `expr.expect("message")`
    Expect {
        expr: Box<RustExpr>,
        message: String,
    },
    /// `&expr`
    Ref(Box<RustExpr>),
    /// `expr.to_string()`
    ToStringCall(Box<RustExpr>),
    /// `serde_json::Value::Null`
    Null,
}
