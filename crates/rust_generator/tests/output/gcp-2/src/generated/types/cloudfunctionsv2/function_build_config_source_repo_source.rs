#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionBuildConfigSourceRepoSource {
    /// Regex matching branches to build.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Option<String>,
    /// Regex matching tags to build.
    #[builder(into)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: Option<String>,
    /// Directory, relative to the source root, in which to run the build.
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: Option<String>,
    /// Only trigger a build if the revision regex does
    /// NOT match the revision regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Option<bool>,
    /// ID of the project that owns the Cloud Source Repository. If omitted, the
    /// project ID requesting the build is assumed.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// Name of the Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Option<String>,
    /// Regex matching tags to build.
    #[builder(into)]
    #[serde(rename = "tagName")]
    pub r#tag_name: Option<String>,
}
