#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryUri {
    /// (Output)
    /// API is the URI for API access.
    #[builder(into)]
    #[serde(rename = "api")]
    pub r#api: Option<String>,
    /// (Output)
    /// git_https is the git HTTPS URI for git operations.
    #[builder(into)]
    #[serde(rename = "gitHttps")]
    pub r#git_https: Option<String>,
    /// (Output)
    /// HTML is the URI for the user to view the repository in a browser.
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: Option<String>,
}
