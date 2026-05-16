use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, parse_macro_input};

pub fn from_pulumi_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let model = crate::pulumi_gestalt_model_path();

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("FromPulumiValue can only be derived for structs with named fields"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_name_strings: Vec<String> = field_names.iter().map(|f| f.to_string()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        impl #impl_generics #model::FromPulumiValue for #name #ty_generics #where_clause {
            fn from_pulumi_value(value: &#model::PulumiValue) -> #model::__private::rootcause::Result<Self> {
                use std::collections::BTreeMap;
                use #model::__private::rootcause::bail;
                use #model::__private::rootcause::prelude::ResultExt;
                use #model::{FromPulumiValue, PulumiValue, PulumiValueContent};

                match value.content {
                    PulumiValueContent::Object(ref obj) => {
                        let fields_map: BTreeMap<String, PulumiValue> = obj.iter().cloned().collect();

                        Ok(Self {
                            #(
                                #field_names: {
                                    let field_value = match fields_map.get(#field_name_strings) {
                                        Some(value) => value,
                                        None => bail!(
                                            "Missing field '{}' while converting PulumiValue to {}",
                                            #field_name_strings,
                                            std::any::type_name::<Self>()
                                        ),
                                    };
                                    <#field_types as FromPulumiValue>::from_pulumi_value(field_value)
                                        .context_with(|| {
                                            format!(
                                                "Failed to convert field '{}' to {}",
                                                #field_name_strings,
                                                std::any::type_name::<#field_types>()
                                            )
                                        })?
                                },
                            )*
                        })
                    }
                    _ => bail!("Expected Object, got {:?}", value.content),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
