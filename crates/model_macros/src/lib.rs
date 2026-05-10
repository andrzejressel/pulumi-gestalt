use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(ToPulumiValue)]
pub fn to_pulumi_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("ToPulumiValue can only be derived for structs with named fields"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_name_strings: Vec<_> = field_names
        .iter()
        .map(|f| f.as_ref().unwrap().to_string())
        .collect();

    let expanded = quote! {
        impl #impl_generics ToPulumiValue for #name #ty_generics #where_clause {
            fn to_pulumi_value(&self) -> impl std::future::Future<Output = PulumiValue> + Clone + Sync + Send {
                use futures::FutureExt;
                #(let #field_names = self.#field_names.to_pulumi_value().boxed();)*

                async move {
                    let results = futures::future::join_all(vec![#(#field_names),*]).await;
                    let mut dependencies = std::collections::HashSet::new();
                    let mut secret = false;
                    let mut content = Vec::new();

                    let mut results_iter = results.into_iter();
                    #(
                        let mut val = results_iter.next().expect("join_all result size mismatch");
                        dependencies.extend(val.dependencies.drain());
                        secret |= val.secret;
                        content.push((#field_name_strings.to_string(), val));
                    )*

                    PulumiValue {
                        content: PulumiValueContent::Object(content),
                        secret,
                        dependencies,
                    }
                }
                .boxed()
                .shared()
            }
        }
    };

    TokenStream::from(expanded)
}
