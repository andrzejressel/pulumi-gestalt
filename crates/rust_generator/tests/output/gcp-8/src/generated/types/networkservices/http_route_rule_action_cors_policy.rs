#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HttpRouteRuleActionCorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials.
    #[builder(into, default)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Box<Option<bool>>,
    /// Specifies the content for Access-Control-Allow-Headers header.
    #[builder(into, default)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Box<Option<Vec<String>>>,
    /// Specifies the content for Access-Control-Allow-Methods header.
    #[builder(into, default)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Box<Option<Vec<String>>>,
    /// Specifies the regular expression patterns that match allowed origins.
    #[builder(into, default)]
    #[serde(rename = "allowOriginRegexes")]
    pub r#allow_origin_regexes: Box<Option<Vec<String>>>,
    /// Specifies the list of origins that will be allowed to do CORS requests.
    #[builder(into, default)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Box<Option<Vec<String>>>,
    /// If true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Specifies the content for Access-Control-Expose-Headers header.
    #[builder(into, default)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Box<Option<Vec<String>>>,
    /// Specifies how long result of a preflight request can be cached in seconds.
    #[builder(into, default)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<String>>,
}
