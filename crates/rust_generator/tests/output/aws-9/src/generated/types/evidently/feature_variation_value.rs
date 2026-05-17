#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureVariationValue {
    /// If this feature uses the Boolean variation type, this field contains the Boolean value of this variation.
    #[builder(into)]
    #[serde(rename = "boolValue")]
    pub r#bool_value: Option<String>,
    /// If this feature uses the double integer variation type, this field contains the double integer value of this variation.
    #[builder(into)]
    #[serde(rename = "doubleValue")]
    pub r#double_value: Option<String>,
    /// If this feature uses the long variation type, this field contains the long value of this variation. Minimum value of `-9007199254740991`. Maximum value of `9007199254740991`.
    #[builder(into)]
    #[serde(rename = "longValue")]
    pub r#long_value: Option<String>,
    /// If this feature uses the string variation type, this field contains the string value of this variation. Minimum length of `0`. Maximum length of `512`.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureVariationValue {
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
                "double_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#double_value,
                )
                .await,
            );
            map.insert(
                "long_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#long_value,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureVariationValue {
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
                    r#double_value: {
                        let field_value = match fields_map.get("double_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'double_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_value: {
                        let field_value = match fields_map.get("long_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
