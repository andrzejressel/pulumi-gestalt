#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketMetricFilter {
    /// S3 Access Point ARN for filtering (singular).
    #[builder(into)]
    #[serde(rename = "accessPoint")]
    pub r#access_point: Option<String>,
    /// Object prefix for filtering (singular).
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Object tags for filtering (up to 10).
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
}
