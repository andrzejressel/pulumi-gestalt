#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
