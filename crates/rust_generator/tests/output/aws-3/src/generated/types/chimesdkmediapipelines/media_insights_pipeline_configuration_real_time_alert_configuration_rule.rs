#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule {
    /// Configuration for an issue detection rule.
    #[builder(into)]
    #[serde(rename = "issueDetectionConfiguration")]
    pub r#issue_detection_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration>>,
    /// Configuration for a keyword match rule.
    #[builder(into)]
    #[serde(rename = "keywordMatchConfiguration")]
    pub r#keyword_match_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration>>,
    /// Configuration for a sentiment rule.
    #[builder(into)]
    #[serde(rename = "sentimentConfiguration")]
    pub r#sentiment_configuration: Option<Box<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration>>,
    /// Rule type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule {
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
                    "issue_detection_configuration",
                    &self.r#issue_detection_configuration,
                ),
                to_pulumi_object_field(
                    "keyword_match_configuration",
                    &self.r#keyword_match_configuration,
                ),
                to_pulumi_object_field(
                    "sentiment_configuration",
                    &self.r#sentiment_configuration,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule {
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
                    r#issue_detection_configuration: {
                        let field_value = match fields_map.get("issue_detection_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'issue_detection_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keyword_match_configuration: {
                        let field_value = match fields_map.get("keyword_match_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyword_match_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sentiment_configuration: {
                        let field_value = match fields_map.get("sentiment_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'sentiment_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
