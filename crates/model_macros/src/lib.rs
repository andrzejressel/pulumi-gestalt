mod from_pulumi_value;
mod to_pulumi_value;

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use syn::{Path, parse_quote};

fn pulumi_gestalt_model_path() -> Path {
    match crate_name("pulumi_gestalt_model") {
        Ok(FoundCrate::Itself) => parse_quote!(crate),
        Ok(FoundCrate::Name(name)) => syn::parse_str(&format!("::{name}")).unwrap(),
        Err(_) => parse_quote!(::pulumi_gestalt_rust::__private::pulumi_gestalt_model),
    }
}

#[proc_macro_derive(ToPulumiValue)]
pub fn to_pulumi_value_derive(input: TokenStream) -> TokenStream {
    to_pulumi_value::to_pulumi_value_derive(input)
}

#[proc_macro_derive(FromPulumiValue)]
pub fn from_pulumi_value_derive(input: TokenStream) -> TokenStream {
    from_pulumi_value::from_pulumi_value_derive(input)
}
