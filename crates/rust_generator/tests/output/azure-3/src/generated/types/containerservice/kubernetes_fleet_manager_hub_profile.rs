#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesFleetManagerHubProfile {
    #[builder(into)]
    #[serde(rename = "dnsPrefix")]
    pub r#dns_prefix: String,
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    #[builder(into)]
    #[serde(rename = "kubernetesVersion")]
    pub r#kubernetes_version: Option<String>,
}
