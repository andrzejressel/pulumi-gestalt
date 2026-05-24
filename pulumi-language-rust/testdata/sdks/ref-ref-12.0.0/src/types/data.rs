#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct Data {
    #[builder(into)]
    pub r#bool_array: Vec<bool>,
    #[builder(into)]
    pub r#boolean: bool,
    #[builder(into)]
    pub r#float: f64,
    #[builder(into)]
    pub r#inner_data: Box<super::types::InnerData>,
    #[builder(into)]
    pub r#integer: i32,
    #[builder(into)]
    pub r#string: String,
    #[builder(into)]
    pub r#string_map: std::collections::BTreeMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Data {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "boolArray",
                    &self.r#bool_array,
                ),
                to_pulumi_object_field(
                    "boolean",
                    &self.r#boolean,
                ),
                to_pulumi_object_field(
                    "float",
                    &self.r#float,
                ),
                to_pulumi_object_field(
                    "innerData",
                    &self.r#inner_data,
                ),
                to_pulumi_object_field(
                    "integer",
                    &self.r#integer,
                ),
                to_pulumi_object_field(
                    "string",
                    &self.r#string,
                ),
                to_pulumi_object_field(
                    "stringMap",
                    &self.r#string_map,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Data {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#bool_array: {
                        let field_value = match fields_map.get("boolArray") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolArray' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boolean: {
                        let field_value = match fields_map.get("boolean") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolean' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#float: {
                        let field_value = match fields_map.get("float") {
                            Some(value) => value,
                            None => bail!("Missing field 'float' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inner_data: {
                        let field_value = match fields_map.get("innerData") {
                            Some(value) => value,
                            None => bail!("Missing field 'innerData' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer: {
                        let field_value = match fields_map.get("integer") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string: {
                        let field_value = match fields_map.get("string") {
                            Some(value) => value,
                            None => bail!("Missing field 'string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_map: {
                        let field_value = match fields_map.get("stringMap") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringMap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
