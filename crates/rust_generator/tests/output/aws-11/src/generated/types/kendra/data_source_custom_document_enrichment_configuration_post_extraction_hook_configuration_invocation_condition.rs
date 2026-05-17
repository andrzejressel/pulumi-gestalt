#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationCondition {
    /// The identifier of the document attribute used for the condition. For example, `_source_uri` could be an identifier for the attribute or metadata field that contains source URIs associated with the documents. Amazon Kendra currently does not support `_document_body` as an attribute key used for the condition.
    #[builder(into)]
    #[serde(rename = "conditionDocumentAttributeKey")]
    pub r#condition_document_attribute_key: String,
    /// The value used by the operator. For example, you can specify the value 'financial' for strings in the `_source_uri` field that partially match or contain this value. See condition_on_value.
    #[builder(into)]
    #[serde(rename = "conditionOnValue")]
    pub r#condition_on_value: Option<Box<super::super::types::kendra::DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationConditionConditionOnValue>>,
    /// The condition operator. For example, you can use `Contains` to partially match a string. Valid Values: `GreaterThan` | `GreaterThanOrEquals` | `LessThan` | `LessThanOrEquals` | `Equals` | `NotEquals` | `Contains` | `NotContains` | `Exists` | `NotExists` | `BeginsWith`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "condition_document_attribute_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#condition_document_attribute_key,
                )
                .await,
            );
            map.insert(
                "condition_on_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#condition_on_value,
                )
                .await,
            );
            map.insert(
                "operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operator,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCustomDocumentEnrichmentConfigurationPostExtractionHookConfigurationInvocationCondition {
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
                    r#condition_document_attribute_key: {
                        let field_value = match fields_map.get("condition_document_attribute_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_document_attribute_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition_on_value: {
                        let field_value = match fields_map.get("condition_on_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_on_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
