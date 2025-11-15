#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceCorsConfiguration {
    /// (Boolean) If credentials are allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowCredentials")]
    pub r#allow_credentials: Option<bool>,
    /// A set of headers to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Option<Vec<String>>,
    /// The methods to be allowed via CORS. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PATCH` and `PUT`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Option<Vec<String>>,
    /// A set of origins to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// The max age to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Option<i32>,
}
