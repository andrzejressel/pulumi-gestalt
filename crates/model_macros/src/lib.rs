mod from_pulumi_value;
mod to_pulumi_value;

use proc_macro::TokenStream;

#[proc_macro_derive(ToPulumiValue)]
pub fn to_pulumi_value_derive(input: TokenStream) -> TokenStream {
    to_pulumi_value::to_pulumi_value_derive(input)
}

#[proc_macro_derive(FromPulumiValue)]
pub fn from_pulumi_value_derive(input: TokenStream) -> TokenStream {
    from_pulumi_value::from_pulumi_value_derive(input)
}
