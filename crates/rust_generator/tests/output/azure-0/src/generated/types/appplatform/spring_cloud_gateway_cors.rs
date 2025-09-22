#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudGatewayCors {
    /// Allowed headers in cross-site requests. The special value `*` allows actual requests to send any header.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// Allowed HTTP methods on cross-site requests. The special value `*` allows all methods. If not set, `GET` and `HEAD` are allowed by default. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS` and `PUT`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    /// Allowed origin patterns to make cross-site requests.
    #[builder(into)]
    #[serde(rename = "allowedOriginPatterns")]
    pub r#allowed_origin_patterns: Option<Vec<String>>,
    /// Allowed origins to make cross-site requests. The special value `*` allows all domains.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// is user credentials are supported on cross-site requests?
    #[builder(into)]
    #[serde(rename = "credentialsAllowed")]
    pub r#credentials_allowed: Option<bool>,
    /// HTTP response headers to expose for cross-site requests.
    #[builder(into)]
    #[serde(rename = "exposedHeaders")]
    pub r#exposed_headers: Option<Vec<String>>,
    /// How long, in seconds, the response from a pre-flight request can be cached by clients.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Option<i32>,
}
