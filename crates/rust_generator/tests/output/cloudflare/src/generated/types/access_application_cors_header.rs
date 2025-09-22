#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessApplicationCorsHeader {
    /// Value to determine whether all HTTP headers are exposed.
    #[builder(into)]
    #[serde(rename = "allowAllHeaders")]
    pub r#allow_all_headers: Option<bool>,
    /// Value to determine whether all methods are exposed.
    #[builder(into)]
    #[serde(rename = "allowAllMethods")]
    pub r#allow_all_methods: Option<bool>,
    /// Value to determine whether all origins are permitted to make CORS requests.
    #[builder(into)]
    #[serde(rename = "allowAllOrigins")]
    pub r#allow_all_origins: Option<bool>,
    /// Value to determine if credentials (cookies, authorization headers, or TLS client certificates) are included with requests.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// List of HTTP headers to expose via CORS.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// List of methods to expose via CORS.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    /// List of origins permitted to make CORS requests.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// The maximum time a preflight request will be cached.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
}
