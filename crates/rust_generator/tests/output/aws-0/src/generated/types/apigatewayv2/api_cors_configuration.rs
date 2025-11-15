#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiCorsConfiguration {
    /// Whether credentials are included in the CORS request.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// Set of allowed HTTP headers.
    #[builder(into)]
    #[serde(rename = "allowHeaders")]
    pub r#allow_headers: Option<Vec<String>>,
    /// Set of allowed HTTP methods.
    #[builder(into)]
    #[serde(rename = "allowMethods")]
    pub r#allow_methods: Option<Vec<String>>,
    /// Set of allowed origins.
    #[builder(into)]
    #[serde(rename = "allowOrigins")]
    pub r#allow_origins: Option<Vec<String>>,
    /// Set of exposed HTTP headers.
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Number of seconds that the browser should cache preflight request results.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}
