#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FhirServiceCors {
    /// A set of headers to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedHeaders")]
    pub r#allowed_headers: Vec<String>,
    /// The methods to be allowed via CORS. Possible values are `DELETE`, `GET`, `HEAD`, `MERGE`, `POST`, `OPTIONS`, `PATCH` and `PUT`.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Vec<String>,
    /// A set of origins to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Vec<String>,
    /// If credentials are allowed via CORS.
    #[builder(into)]
    #[serde(rename = "credentialsAllowed")]
    pub r#credentials_allowed: Option<bool>,
    /// The max age to be allowed via CORS.
    #[builder(into)]
    #[serde(rename = "maxAgeInSeconds")]
    pub r#max_age_in_seconds: Option<i32>,
}
