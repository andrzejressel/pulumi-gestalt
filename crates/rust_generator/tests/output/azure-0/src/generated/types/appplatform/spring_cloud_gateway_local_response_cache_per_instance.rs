#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudGatewayLocalResponseCachePerInstance {
    /// Specifies the maximum size of cache (10MB, 900KB, 1GB...) to determine if the cache needs to evict some entries.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<String>,
    /// Specifies the time before a cached entry is expired (300s, 5m, 1h...).
    #[builder(into)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Option<String>,
}
