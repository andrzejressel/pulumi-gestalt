use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, parse_macro_input};

pub fn to_pulumi_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let model = crate::pulumi_gestalt_model_path();

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("ToPulumiValue can only be derived for structs with named fields"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_name_strings: Vec<String> = field_names
        .iter()
        .map(|f| f.as_ref().unwrap().to_string())
        .collect();

    let expanded = quote! {
        impl #impl_generics #model::ToPulumiValue for #name #ty_generics #where_clause {
            fn to_pulumi_value(&self) -> impl std::future::Future<Output = #model::PulumiValue> + Clone + Sync + Send {
                use #model::__private::futures::FutureExt;
                use std::collections::BTreeMap;
                use #model::PulumiValue;
                use #model::ToPulumiValue;
                #(let #field_names = self.#field_names.to_pulumi_value().boxed();)*

                async move {
                    let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
                    #(
                        map.insert(
                            #field_name_strings.clone().to_string(),
                            #field_names.await,
                        );
                    )*
                    map.to_pulumi_value().await
                }
                .boxed()
                .shared()
            }
        }
    };

    TokenStream::from(expanded)
}
