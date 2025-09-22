#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationActions {
    /// Action to take for a high risk. See action block below.
    #[builder(into)]
    #[serde(rename = "highAction")]
    pub r#high_action: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction>>,
    /// Action to take for a low risk. See action block below.
    #[builder(into)]
    #[serde(rename = "lowAction")]
    pub r#low_action: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction>>,
    /// Action to take for a medium risk. See action block below.
    #[builder(into)]
    #[serde(rename = "mediumAction")]
    pub r#medium_action: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction>>,
}
