#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterIngressProfile {
    /// The IP Address the Ingress Profile is associated with.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The name of the Azure Red Hat OpenShift Cluster to create. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Cluster Ingress visibility. Supported values are `Public` and `Private`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: String,
}
