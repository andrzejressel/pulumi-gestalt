#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipPolicycontroller {
    /// Policy Controller configuration for the cluster. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyControllerHubConfig")]
    pub r#policy_controller_hub_config: Box<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfig>,
    /// Version of Policy Controller to install. Defaults to the latest version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
