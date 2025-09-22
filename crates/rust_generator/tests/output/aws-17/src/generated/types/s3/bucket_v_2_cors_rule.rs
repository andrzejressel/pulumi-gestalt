#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketV2CorsRule {
    /// List of headers allowed.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// One or more HTTP methods that you allow the origin to execute. Can be `GET`, `PUT`, `POST`, `DELETE` or `HEAD`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Vec<String>,
    /// One or more origins you want customers to be able to access the bucket from.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Vec<String>,
    /// One or more headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript `XMLHttpRequest` object).
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Specifies time in seconds that browser can cache the response for a preflight request.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Option<i32>,
}
