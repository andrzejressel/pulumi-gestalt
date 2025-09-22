#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration {
    /// Matches a taint effect.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Option<String>,
    /// Matches a taint key (not necessarily unique).
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Matches a taint operator.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// Matches a taint value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
