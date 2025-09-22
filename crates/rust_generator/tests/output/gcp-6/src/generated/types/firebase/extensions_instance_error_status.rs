#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExtensionsInstanceErrorStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<i32>,
    /// A list of messages that carry the error details.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Option<Vec<std::collections::HashMap<String, String>>>,
    /// A developer-facing error message, which should be in English.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}
