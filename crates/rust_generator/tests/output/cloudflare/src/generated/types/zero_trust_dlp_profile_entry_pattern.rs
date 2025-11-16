#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustDlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: String,
    /// The validation algorithm to apply with this pattern.
    #[builder(into)]
    #[serde(rename = "validation")]
    pub r#validation: Option<String>,
}
