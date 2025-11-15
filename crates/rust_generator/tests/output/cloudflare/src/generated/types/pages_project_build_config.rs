#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PagesProjectBuildConfig {
    /// Enable build caching for the project.
    #[builder(into)]
    #[serde(rename = "buildCaching")]
    pub r#build_caching: Option<bool>,
    /// Command used to build project.
    #[builder(into)]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Option<String>,
    /// Output directory of the build.
    #[builder(into)]
    #[serde(rename = "destinationDir")]
    pub r#destination_dir: Option<String>,
    /// Your project's root directory, where Cloudflare runs the build command. If your site is not in a subdirectory, leave this path value empty.
    #[builder(into)]
    #[serde(rename = "rootDir")]
    pub r#root_dir: Option<String>,
    /// The classifying tag for analytics.
    #[builder(into)]
    #[serde(rename = "webAnalyticsTag")]
    pub r#web_analytics_tag: Option<String>,
    /// The auth token for analytics.
    #[builder(into)]
    #[serde(rename = "webAnalyticsToken")]
    pub r#web_analytics_token: Option<String>,
}
