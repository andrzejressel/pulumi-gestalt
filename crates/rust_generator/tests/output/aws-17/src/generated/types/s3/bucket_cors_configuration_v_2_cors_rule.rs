#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketCorsConfigurationV2CorsRule {
    /// Set of Headers that are specified in the `Access-Control-Request-Headers` header.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// Set of HTTP methods that you allow the origin to execute. Valid values are `GET`, `PUT`, `HEAD`, `POST`, and `DELETE`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Vec<String>,
    /// Set of origins you want customers to be able to access the bucket from.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Vec<String>,
    /// Set of headers in the response that you want customers to be able to access from their applications (for example, from a JavaScript `XMLHttpRequest` object).
    #[builder(into)]
    #[serde(rename = "exposeHeaders")]
    pub r#expose_headers: Option<Vec<String>>,
    /// Unique identifier for the rule. The value cannot be longer than 255 characters.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Time in seconds that your browser is to cache the preflight response for the specified resource.
    #[builder(into)]
    #[serde(rename = "maxAgeSeconds")]
    pub r#max_age_seconds: Option<i32>,
}
