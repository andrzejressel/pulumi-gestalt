#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentNetworkFunctionGroup {
    /// Optional description of the network function group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// This identifies the network function group container.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// This will be either `true`, that attachment acceptance is required, or `false`, that it is not required.
    #[builder(into)]
    #[serde(rename = "requireAttachmentAcceptance")]
    pub r#require_attachment_acceptance: bool,
}
