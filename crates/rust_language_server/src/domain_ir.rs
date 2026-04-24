use pulumi_gestalt_schema::model::ElementId;

/// Domain IR: language-agnostic Pulumi program representation.
///
/// This layer captures the *semantic intent* of a Pulumi program without any
/// knowledge of the target language (Rust). It sits between the PCL IR (which
/// mirrors the protobuf wire format) and the Rust IR (which mirrors Rust syntax).

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum Statement {
    ConfigBinding(ConfigBinding),
    LocalBinding {
        name: String,
        value: Expr,
    },
    Export {
        name: String,
        value: Expr,
    },
    RequirePulumiVersion(Expr),
    Resource {
        /// Rust variable name (snake_case), e.g. `my_stash`.
        name: String,
        /// Pulumi logical name used in the API call, e.g. `"myStash"`.
        logical_name: String,
        token: ResourceToken,
        /// Input fields: `(field_name, value)`.
        inputs: Vec<ResourceInput>,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ResourceInput {
    pub name: String,
    pub expression: Expr,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum ResourceToken {
    Stash,
    Custom {
        provider_name: String,
        element_id: ElementId,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ConfigBinding {
    pub name: String,
    pub config_type: ConfigType,
    pub default: Option<Expr>,
    pub secret: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum ConfigType {
    String,
    Number,
    Int,
    Bool,
    List(Box<ConfigType>),
    Map(Box<ConfigType>),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum Expr {
    // Literals
    String(String),
    Number(f64),
    Bool(bool),
    Null,

    // Structural
    Variable(String),
    FieldAccess(Box<Expr>, String),
    IndexAccess(Box<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Format {
        parts: Vec<Expr>,
    },

    // Pulumi-specific
    OutputMap {
        output: Box<Expr>,
        params: Vec<String>,
        body: Box<Expr>,
    },
    CombineOutputs {
        outputs: Vec<Expr>,
        params: Vec<String>,
        body: Box<Expr>,
    },
    MakeSecret(Box<Expr>),
    MakeUnsecret(Box<Expr>),
    NewSecret(Box<Expr>),
    NewOutput(Box<Expr>),
    PulumiAny(JsonValue),

    // Stdlib
    StdlibCall {
        func: StdlibFn,
        args: Vec<Expr>,
    },

    // Generic
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Closure {
        params: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    Expr(Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum StdlibFn {
    FromBase64,
    ToBase64,
    Sha1,
    ReadFile,
    FileBase64,
    FileBase64Sha256,
    Element,
    Join,
    Length,
    Split,
    SingleOrNone,
    Cwd,
    RootDirectory,
    Stack,
    Organization,
    Project,
    Entries,
    Lookup,
    Min,
    Max,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Or,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum UnaryOp {
    Not,
    Neg,
}
