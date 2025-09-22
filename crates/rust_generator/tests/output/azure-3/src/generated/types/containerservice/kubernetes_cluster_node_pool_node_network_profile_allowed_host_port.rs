#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterNodePoolNodeNetworkProfileAllowedHostPort {
    /// Specifies the end of the port range.
    #[builder(into)]
    #[serde(rename = "portEnd")]
    pub r#port_end: Option<i32>,
    /// Specifies the start of the port range.
    #[builder(into)]
    #[serde(rename = "portStart")]
    pub r#port_start: Option<i32>,
    /// Specifies the protocol of the port range. Possible values are `TCP` and `UDP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}
