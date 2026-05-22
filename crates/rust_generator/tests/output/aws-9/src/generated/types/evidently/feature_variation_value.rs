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
                    "doubleValue",
                    &self.r#double_value,
                ),
                to_pulumi_object_field(
                    "longValue",
                    &self.r#long_value,
                ),
                to_pulumi_object_field(
                    "stringValue",
                    &self.r#string_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("boolValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'boolValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#long_value: {
                        let field_value = match fields_map.get("longValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'longValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
