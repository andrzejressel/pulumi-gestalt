#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxTestCaseTestCaseConversationTurnVirtualAgentOutputCurrentPage {
    /// (Output)
    /// The human-readable name of the page, unique within the flow.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The unique identifier of the page.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
