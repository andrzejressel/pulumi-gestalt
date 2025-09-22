#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterServiceExternalIpsConfig {
    /// Controls whether external ips specified by a service will be allowed. It is enabled by default.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}
