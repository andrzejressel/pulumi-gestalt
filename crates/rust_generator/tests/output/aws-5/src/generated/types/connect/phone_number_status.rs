#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PhoneNumberStatus {
    /// The status message.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The status of the phone number. Valid Values: `CLAIMED` | `IN_PROGRESS` | `FAILED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
