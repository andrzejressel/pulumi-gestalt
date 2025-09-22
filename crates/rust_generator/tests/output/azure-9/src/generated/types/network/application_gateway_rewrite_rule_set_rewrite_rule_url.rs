#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayRewriteRuleSetRewriteRuleUrl {
    /// The components used to rewrite the URL. Possible values are `path_only` and `query_string_only` to limit the rewrite to the URL Path or URL Query String only.
    /// 
    /// > **Note:** One or both of `path` and `query_string` must be specified. If one of these is not specified, it means the value will be empty. If you only want to rewrite `path` or `query_string`, use `components`.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<String>,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The query string to rewrite.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<String>,
    /// Whether the URL path map should be reevaluated after this rewrite has been applied. [More info on rewrite configuration](https://docs.microsoft.com/azure/application-gateway/rewrite-http-headers-url#rewrite-configuration)
    #[builder(into)]
    #[serde(rename = "reroute")]
    pub r#reroute: Option<bool>,
}
