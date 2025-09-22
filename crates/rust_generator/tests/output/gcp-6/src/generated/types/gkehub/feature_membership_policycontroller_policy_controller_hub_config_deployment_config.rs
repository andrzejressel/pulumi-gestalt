#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfig {
    /// The name of the component. One of `admission` `audit` or `mutation`
    #[builder(into)]
    #[serde(rename = "componentName")]
    pub r#component_name: String,
    /// Container resource requirements.
    #[builder(into)]
    #[serde(rename = "containerResources")]
    pub r#container_resources: Option<Box<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources>>,
    /// Pod affinity configuration. Possible values: AFFINITY_UNSPECIFIED, NO_AFFINITY, ANTI_AFFINITY
    #[builder(into)]
    #[serde(rename = "podAffinity")]
    pub r#pod_affinity: Option<String>,
    /// Pod tolerations of node taints.
    #[builder(into)]
    #[serde(rename = "podTolerations")]
    pub r#pod_tolerations: Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigDeploymentConfigPodToleration>>,
    /// Pod replica count.
    #[builder(into)]
    #[serde(rename = "replicaCount")]
    pub r#replica_count: Option<i32>,
}
