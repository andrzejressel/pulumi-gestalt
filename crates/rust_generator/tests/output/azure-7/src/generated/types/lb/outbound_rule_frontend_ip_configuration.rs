#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OutboundRuleFrontendIpConfiguration {
    /// The ID of the Load Balancer Outbound Rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
