#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontroller {
    /// Configuration of Policy Controller
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyControllerHubConfig")]
    pub r#policy_controller_hub_config: Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfig>,
    /// Configures the version of Policy Controller
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
