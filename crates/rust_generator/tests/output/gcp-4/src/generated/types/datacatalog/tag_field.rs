#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagField {
    /// Holds the value for a tag field with boolean type.
    #[builder(into)]
    #[serde(rename = "boolValue")]
    pub r#bool_value: Option<bool>,
    /// (Output)
    /// The display name of this field
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Holds the value for a tag field with double type.
    #[builder(into)]
    #[serde(rename = "doubleValue")]
    pub r#double_value: Option<f64>,
    /// Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "enumValue")]
    pub r#enum_value: Option<String>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fieldName")]
    pub r#field_name: String,
    /// (Output)
    /// The order of this field with respect to other fields in this tag. For example, a higher value can indicate
    /// a more important field. The value can be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// Holds the value for a tag field with string type.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
    /// Holds the value for a tag field with timestamp type.
    #[builder(into)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TagField {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bool_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bool_value,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "double_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#double_value,
                )
                .await,
            );
            map.insert(
                "enum_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enum_value,
                )
                .await,
            );
            map.insert(
                "field_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_name,
                )
                .await,
            );
            map.insert(
                "order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#order,
                )
                .await,
            );
            map.insert(
                "string_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#string_value,
                )
                .await,
            );
            map.insert(
                "timestamp_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timestamp_value,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("bool_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'bool_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#double_value: {
                        let field_value = match fields_map.get("double_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'double_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_value: {
                        let field_value = match fields_map.get("enum_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'enum_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_name: {
                        let field_value = match fields_map.get("field_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("string_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_value: {
                        let field_value = match fields_map.get("timestamp_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
