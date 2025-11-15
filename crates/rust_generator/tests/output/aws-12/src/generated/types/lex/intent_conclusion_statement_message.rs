#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntentConclusionStatementMessage {
    /// The text of the message. Must be less than or equal to 1000 characters in length.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: String,
    /// The content type of the message string.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// Identifies the message group that the message belongs to. When a group
    /// is assigned to a message, Amazon Lex returns one message from each group in the response. Must be a number between 1 and 5 (inclusive).
    #[builder(into)]
    #[serde(rename = "groupNumber")]
    pub r#group_number: Option<i32>,
}
