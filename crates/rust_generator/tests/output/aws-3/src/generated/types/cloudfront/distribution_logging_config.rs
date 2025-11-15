#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionLoggingConfig {
    /// Amazon S3 bucket to store the access logs in, for example, `myawslogbucket.s3.amazonaws.com`.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Whether to include cookies in access logs (default: `false`).
    #[builder(into)]
    #[serde(rename = "includeCookies")]
    pub r#include_cookies: Option<bool>,
    /// Prefix to the access log filenames for this distribution, for example, `myprefix/`.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
