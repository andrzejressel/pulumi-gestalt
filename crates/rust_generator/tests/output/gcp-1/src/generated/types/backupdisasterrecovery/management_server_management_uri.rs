#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagementServerManagementUri {
    /// (Output)
    /// The management console api endpoint.
    #[builder(into)]
    #[serde(rename = "api")]
    pub r#api: Option<String>,
    /// (Output)
    /// The management console webUi.
    #[builder(into)]
    #[serde(rename = "webUi")]
    pub r#web_ui: Option<String>,
}
