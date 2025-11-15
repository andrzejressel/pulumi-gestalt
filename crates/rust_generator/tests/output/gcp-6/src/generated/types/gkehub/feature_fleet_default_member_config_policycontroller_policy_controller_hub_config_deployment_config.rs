#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "component")]
    pub r#component: String,
    /// Container resource requirements.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containerResources")]
    pub r#container_resources: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration.
    /// Possible values are: `AFFINITY_UNSPECIFIED`, `NO_AFFINITY`, `ANTI_AFFINITY`.
    #[builder(into)]
    #[serde(rename = "podAffinity")]
    pub r#pod_affinity: Option<String>,
    /// Pod tolerations of node taints.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "podTolerations")]
    pub r#pod_tolerations: Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>,
    /// Pod replica count.
    #[builder(into)]
    #[serde(rename = "replicaCount")]
    pub r#replica_count: Option<i32>,
}
