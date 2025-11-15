#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingCustomDomainCertVerificationHttp {
    /// A text string to serve at the path.
    #[builder(into)]
    #[serde(rename = "desired")]
    pub r#desired: Option<String>,
    /// Whether Hosting was able to find the required file contents on the
    /// specified path during its last check.
    #[builder(into)]
    #[serde(rename = "discovered")]
    pub r#discovered: Option<String>,
    /// (Output)
    /// The last time Hosting systems checked for the file contents.
    #[builder(into)]
    #[serde(rename = "lastCheckTime")]
    pub r#last_check_time: Option<String>,
    /// The path to the file.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}
