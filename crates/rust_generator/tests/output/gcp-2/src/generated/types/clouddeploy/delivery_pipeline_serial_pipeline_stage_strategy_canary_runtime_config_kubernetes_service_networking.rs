#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesServiceNetworking {
    /// Required. Name of the Kubernetes Deployment whose traffic is managed by the specified Service.
    #[builder(into)]
    #[serde(rename = "deployment")]
    pub r#deployment: String,
    /// Optional. Whether to disable Pod overprovisioning. If Pod overprovisioning is disabled then Cloud Deploy will limit the number of total Pods used for the deployment strategy to the number of Pods the Deployment has on the cluster.
    #[builder(into)]
    #[serde(rename = "disablePodOverprovisioning")]
    pub r#disable_pod_overprovisioning: Option<bool>,
    /// Optional. The label to use when selecting Pods for the Deployment resource. This label must already be present in the Deployment.
    #[builder(into)]
    #[serde(rename = "podSelectorLabel")]
    pub r#pod_selector_label: Option<String>,
    /// Required. Name of the Kubernetes Service.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}
