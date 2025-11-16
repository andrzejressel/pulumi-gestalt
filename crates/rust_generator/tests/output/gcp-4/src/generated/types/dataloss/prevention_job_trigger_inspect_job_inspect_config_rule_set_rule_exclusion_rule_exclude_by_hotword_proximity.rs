#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeByHotwordProximity {
    /// Number of characters after the finding to consider. Either this or window_before must be specified
    #[builder(into)]
    #[serde(rename = "windowAfter")]
    pub r#window_after: Option<i32>,
    /// Number of characters before the finding to consider. Either this or window_after must be specified
    #[builder(into)]
    #[serde(rename = "windowBefore")]
    pub r#window_before: Option<i32>,
}
