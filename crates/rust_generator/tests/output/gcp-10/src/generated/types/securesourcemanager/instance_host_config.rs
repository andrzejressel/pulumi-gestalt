#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceHostConfig {
    /// (Output)
    /// API hostname.
    #[builder(into)]
    #[serde(rename = "api")]
    pub r#api: Option<String>,
    /// (Output)
    /// Git HTTP hostname.
    #[builder(into)]
    #[serde(rename = "gitHttp")]
    pub r#git_http: Option<String>,
    /// (Output)
    /// Git SSH hostname.
    #[builder(into)]
    #[serde(rename = "gitSsh")]
    pub r#git_ssh: Option<String>,
    /// (Output)
    /// HTML hostname.
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: Option<String>,
}
