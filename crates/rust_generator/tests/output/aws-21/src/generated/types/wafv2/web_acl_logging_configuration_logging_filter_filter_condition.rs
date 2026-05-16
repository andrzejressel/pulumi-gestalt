#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclLoggingConfigurationLoggingFilterFilterCondition {
    /// Configuration for a single action condition. See Action Condition below for more details.
    #[builder(into)]
    #[serde(rename = "actionCondition")]
    pub r#action_condition: Option<Box<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition>>,
    /// Condition for a single label name. See Label Name Condition below for more details.
    #[builder(into)]
    #[serde(rename = "labelNameCondition")]
    pub r#label_name_condition: Option<Box<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionLabelNameCondition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclLoggingConfigurationLoggingFilterFilterCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action_condition".to_string(), self.r#action_condition.to_pulumi_value().await);
            map.insert("label_name_condition".to_string(), self.r#label_name_condition.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclLoggingConfigurationLoggingFilterFilterCondition {
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
                    r#action_condition: {
                        let field_value = match fields_map.get("action_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#label_name_condition: {
                        let field_value = match fields_map.get("label_name_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_name_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterConditionLabelNameCondition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
