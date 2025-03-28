#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterAddonsConfigIstioConfig {
    /// The authentication type between services in Istio. Available options include `AUTH_MUTUAL_TLS`.
    #[builder(into, default)]
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    /// The status of the Istio addon, which makes it easy to set up Istio for services in a
    /// cluster. It is disabled by default. Set `disabled = false` to enable.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<bool>,
}
