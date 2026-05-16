#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetUpdateRunManagedClusterUpdateUpgrade {
    /// Specifies the Kubernetes version to upgrade the member clusters to. This is required if `type` is set to `Full`.
    #[builder(into)]
    #[serde(rename = "kubernetesVersion")]
    pub r#kubernetes_version: Option<String>,
    /// Specifies the type of upgrade to perform. Possible values are `Full` and `NodeImageOnly`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
