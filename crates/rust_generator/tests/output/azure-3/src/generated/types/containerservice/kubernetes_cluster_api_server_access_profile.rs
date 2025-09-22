#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterApiServerAccessProfile {
    /// Set of authorized IP ranges to allow access to API server, e.g. ["198.51.100.0/24"].
    #[builder(into)]
    #[serde(rename = "authorizedIpRanges")]
    pub r#authorized_ip_ranges: Option<Vec<String>>,
}
