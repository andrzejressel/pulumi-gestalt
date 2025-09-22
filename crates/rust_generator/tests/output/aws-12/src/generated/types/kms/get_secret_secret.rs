#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecretSecret {
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "grantTokens")]
    pub r#grant_tokens: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: String,
}
