#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResources {
    /// Limits describes the maximum amount of compute resources allowed for use by the running container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResourcesLimits>>,
    /// Requests describes the amount of compute resources reserved for the container by the kube-scheduler.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigDeploymentConfigContainerResourcesRequests>>,
}
