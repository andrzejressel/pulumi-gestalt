#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionCacheBehavior {
    /// The cache behavior for the specified path.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: Box<String>,
    /// The path to a directory or file to cached, or not cache. Use an asterisk symbol to specify wildcard directories (path/to/assets/\*), and file types (\*.html, \*jpg, \*js). Directories and file paths are case-sensitive.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
