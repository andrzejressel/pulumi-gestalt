#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue {
    /// A date expressed as an ISO 8601 string. It is important for the time zone to be included in the ISO 8601 date-time format. As of this writing only UTC is supported. For example, `2012-03-25T12:30:10+00:00`.
    #[builder(into)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Option<String>,
    /// A long integer value.
    #[builder(into)]
    #[serde(rename = "longValue")]
    pub r#long_value: Option<i32>,
    /// A list of strings.
    #[builder(into)]
    #[serde(rename = "stringListValues")]
    pub r#string_list_values: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue {
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
                    "date_value",
                    &self.r#date_value,
                ),
                to_pulumi_object_field(
                    "long_value",
                    &self.r#long_value,
                ),
                to_pulumi_object_field(
                    "string_list_values",
                    &self.r#string_list_values,
                ),
                to_pulumi_object_field(
                    "string_value",
                    &self.r#string_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationTargetTargetDocumentAttributeValue {
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
                    r#date_value: {
                        let field_value = match fields_map.get("date_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#string_list_values: {
                        let field_value = match fields_map.get("string_list_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_list_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
