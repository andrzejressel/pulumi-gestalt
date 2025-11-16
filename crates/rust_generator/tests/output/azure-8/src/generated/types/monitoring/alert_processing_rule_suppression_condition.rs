#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertProcessingRuleSuppressionCondition {
    /// A `alert_context` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertContext")]
    pub r#alert_context: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionAlertContext>>,
    /// A `alert_rule_id` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertRuleId")]
    pub r#alert_rule_id: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionAlertRuleId>>,
    /// A `alert_rule_name` block as defined above.
    #[builder(into)]
    #[serde(rename = "alertRuleName")]
    pub r#alert_rule_name: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionAlertRuleName>>,
    /// A `description` block as defined below.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionDescription>>,
    /// A `monitor_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "monitorCondition")]
    pub r#monitor_condition: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionMonitorCondition>>,
    /// A `monitor_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "monitorService")]
    pub r#monitor_service: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionMonitorService>>,
    /// A `severity` block as defined below.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionSeverity>>,
    /// A `signal_type` block as defined below.
    #[builder(into)]
    #[serde(rename = "signalType")]
    pub r#signal_type: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionSignalType>>,
    /// A `target_resource` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionTargetResource>>,
    /// A `target_resource_group` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetResourceGroup")]
    pub r#target_resource_group: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionTargetResourceGroup>>,
    /// A `target_resource_type` block as defined below.
    #[builder(into)]
    #[serde(rename = "targetResourceType")]
    pub r#target_resource_type: Option<Box<super::super::types::monitoring::AlertProcessingRuleSuppressionConditionTargetResourceType>>,
}
