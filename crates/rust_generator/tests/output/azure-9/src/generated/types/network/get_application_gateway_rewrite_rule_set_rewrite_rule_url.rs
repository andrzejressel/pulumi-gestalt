#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayRewriteRuleSetRewriteRuleUrl {
    /// The components used to rewrite the URL.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: String,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The query string to rewrite.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: String,
    /// Whether the URL path map is reevaluated after this rewrite has been applied.
    #[builder(into)]
    #[serde(rename = "reroute")]
    pub r#reroute: bool,
}
