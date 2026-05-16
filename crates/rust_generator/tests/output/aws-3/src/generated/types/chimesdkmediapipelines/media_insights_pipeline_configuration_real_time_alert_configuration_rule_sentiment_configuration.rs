#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration {
    /// Rule name.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: String,
    /// Sentiment type to match.
    #[builder(into)]
    #[serde(rename = "sentimentType")]
    pub r#sentiment_type: String,
    /// Analysis interval.
    #[builder(into)]
    #[serde(rename = "timePeriod")]
    pub r#time_period: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("rule_name".to_string(), self.r#rule_name.to_pulumi_value().await);
            map.insert("sentiment_type".to_string(), self.r#sentiment_type.to_pulumi_value().await);
            map.insert("time_period".to_string(), self.r#time_period.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#rule_name: {
                        let field_value = match fields_map.get("rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sentiment_type: {
                        let field_value = match fields_map.get("sentiment_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'sentiment_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#time_period: {
                        let field_value = match fields_map.get("time_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
