#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterWorkloadAutoscalerProfile {
    /// Specifies whether KEDA Autoscaler can be used for workloads.
    #[builder(into)]
    #[serde(rename = "kedaEnabled")]
    pub r#keda_enabled: Option<bool>,
    /// Specifies whether Vertical Pod Autoscaler should be enabled.
    /// 
    /// > **Note:** This requires that the Preview Feature `Microsoft.ContainerService/AKS-VPAPreview` is enabled and the Resource Provider is re-registered, see the documentation for more information.
    #[builder(into)]
    #[serde(rename = "verticalPodAutoscalerEnabled")]
    pub r#vertical_pod_autoscaler_enabled: Option<bool>,
}
