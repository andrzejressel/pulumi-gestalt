use crate::description::Description;
use anyhow::Context;
use convert_case::{Case, Casing};
use pulumi_gestalt_schema::model::Package;
use regex::Regex;

pub(crate) fn replace_multiple_dashes(s: &str) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn sanitize_rust_identifier(input: &str) -> String {
    input
        .replace('\'', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
        .to_case(Case::UpperCamel)
}

pub(crate) fn escape_rust_name(name: &str) -> &str {
    match name {
        // Escape Rust keywords.
        // Source: https://doc.rust-lang.org/reference/keywords.html
        "as" => "as_",
        "break" => "break_",
        "const" => "const_",
        "continue" => "continue_",
        "crate" => "crate_",
        "else" => "else_",
        "enum" => "enum_",
        "extern" => "extern_",
        "false" => "false_",
        "fn" => "fn_",
        "for" => "for_",
        "if" => "if_",
        "impl" => "impl_",
        "in" => "in_",
        "let" => "let_",
        "loop" => "loop_",
        "match" => "match_",
        "mod" => "mod_",
        "move" => "move_",
        "mut" => "mut_",
        "pub" => "pub_",
        "ref" => "ref_",
        "return" => "return_",
        "self" => "self_",
        "static" => "static_",
        "struct" => "struct_",
        "super" => "super_",
        "trait" => "trait_",
        "true" => "true_",
        "type" => "type_",
        "unsafe" => "unsafe_",
        "use" => "use_",
        "where" => "where_",
        "while" => "while_",
        "async" => "async_",
        "await" => "await_",
        "dyn" => "dyn_",
        "abstract" => "abstract_",
        "become" => "become_",
        "box" => "box_",
        "do" => "do_",
        "final" => "final_",
        "macro" => "macro_",
        "override" => "override_",
        "priv" => "priv_",
        "typeof" => "typeof_",
        "unsized" => "unsized_",
        "virtual" => "virtual_",
        "yield" => "yield_",
        "try" => "try_",
        s => s,
    }
}

pub(crate) fn to_lines(s: Option<String>, package: &Package) -> Vec<String> {
    let binding = s.unwrap_or("".to_string());
    let lines = binding.lines();

    let mut description = Description::create(package);

    for line in lines {
        description.transition(line)
    }

    description.get()
}

pub(crate) fn reformat_code(code: &str) -> anyhow::Result<String> {
    let syntax_tree = syn::parse_file(code)
        .with_context(|| code.to_string())
        .with_context(|| "Failed to parse code".to_string())?;
    Ok(prettyplease::unparse(&syntax_tree))
}

pub(crate) fn access_root(depth: usize) -> String {
    if depth > 0 {
        "super::".repeat(depth)
    } else {
        "self::".to_string()
    }
}
