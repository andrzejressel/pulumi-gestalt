#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingVersionConfigRewrite {
    /// The function to proxy requests to. Must match the exported function name exactly.
    #[builder(into)]
    #[serde(rename = "function")]
    pub r#function: Option<String>,
    /// The user-supplied glob to match against the request URL path.
    #[builder(into)]
    #[serde(rename = "glob")]
    pub r#glob: Option<String>,
    /// The URL path to rewrite the request to.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<String>,
    /// The request will be forwarded to Cloud Run.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "run")]
    pub r#run: Option<Box<super::super::types::firebase::HostingVersionConfigRewriteRun>>,
}
