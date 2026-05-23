#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagField {
    /// Holds the value for a tag field with boolean type.
    #[builder(into)]
    pub r#bool_value: Option<bool>,
    /// (Output)
    /// The display name of this field
    #[builder(into)]
    pub r#display_name: Option<String>,
    /// Holds the value for a tag field with double type.
    #[builder(into)]
    pub r#double_value: Option<f64>,
    /// Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum.
    /// 
    /// - - -
    #[builder(into)]
    pub r#enum_value: Option<String>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    pub r#field_name: String,
    /// (Output)
    /// The order of this field with respect to other fields in this tag. For example, a higher value can indicate
    /// a more important field. The value can be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[builder(into)]
    pub r#order: Option<i32>,
    /// Holds the value for a tag field with string type.
    #[builder(into)]
    pub r#string_value: Option<String>,
    /// Holds the value for a tag field with timestamp type.
    #[builder(into)]
    pub r#timestamp_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TagField {
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
                    "boolValue",
                    &self.r#bool_value,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "doubleValue",
                    &self.r#double_value,
                ),
                to_pulumi_object_field(
                    "enumValue",
                    &self.r#enum_value,
                ),
                to_pulumi_object_field(
                    "fieldName",
                    &self.r#field_name,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "stringValue",
                    &self.r#string_value,
                ),
                to_pulumi_object_field(
                    "timestampValue",
                    &self.r#timestamp_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TagField {
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
                    r#bool_value: {
                        let field_value = match fields_map.get("boolValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#double_value: {
                        let field_value = match fields_map.get("doubleValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'doubleValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_value: {
                        let field_value = match fields_map.get("enumValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'enumValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_name: {
                        let field_value = match fields_map.get("fieldName") {
                            Some(value) => value,
                            None => bail!("Missing field 'fieldName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#string_value: {
                        let field_value = match fields_map.get("stringValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'stringValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_value: {
                        let field_value = match fields_map.get("timestampValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
