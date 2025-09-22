#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NfsFileShareCacheAttributes {
    /// Refreshes a file share's cache by using Time To Live (TTL).
    /// TTL is the length of time since the last refresh after which access to the directory would cause the file gateway
    /// to first refresh that directory's contents from the Amazon S3 bucket. Valid Values: 300 to 2,592,000 seconds (5 minutes to 30 days)
    #[builder(into)]
    #[serde(rename = "cacheStaleTimeoutInSeconds")]
    pub r#cache_stale_timeout_in_seconds: Option<i32>,
}
