#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// Specifies the content for Access-Control-Allow-Headers header.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Option<Vec<String>>,
    /// Specifies the content for Access-Control-Allow-Methods header.
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Option<Vec<String>>,
    /// Specifies the regular expression patterns that match allowed origins.
    #[builder(into)]
    #[serde(rename = "allowOriginRegexes")]
    pub r#allow_origin_regexes: Option<Vec<String>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Option<Vec<String>>,
    /// If true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Specifies the content for Access-Control-Expose-Headers header.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Specifies how long result of a preflight request can be cached in seconds.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<String>,
}
