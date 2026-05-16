#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterMonitorMetrics {
    /// Specifies a comma-separated list of Kubernetes annotation keys that will be used in the resource's labels metric.
    #[builder(into)]
    #[serde(rename = "annotationsAllowed")]
    pub r#annotations_allowed: Option<String>,
    /// Specifies a Comma-separated list of additional Kubernetes label keys that will be used in the resource's labels metric.
    /// 
    /// > **Note:** Both properties `annotations_allowed` and `labels_allowed` are required if you are enabling Managed Prometheus with an existing Azure Monitor Workspace.
    #[builder(into)]
    #[serde(rename = "labelsAllowed")]
    pub r#labels_allowed: Option<String>,
}
