#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionCustomErrorResponse {
    /// Minimum amount of time you want HTTP error codes to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated.
    #[builder(into)]
    #[serde(rename = "errorCachingMinTtl")]
    pub r#error_caching_min_ttl: Option<i32>,
    /// 4xx or 5xx HTTP status code that you want to customize.
    #[builder(into)]
    #[serde(rename = "errorCode")]
    pub r#error_code: i32,
    /// HTTP status code that you want CloudFront to return with the custom error page to the viewer.
    #[builder(into)]
    #[serde(rename = "responseCode")]
    pub r#response_code: Option<i32>,
    /// Path of the custom error page (for example, `/custom_404.html`).
    #[builder(into)]
    #[serde(rename = "responsePagePath")]
    pub r#response_page_path: Option<String>,
}
