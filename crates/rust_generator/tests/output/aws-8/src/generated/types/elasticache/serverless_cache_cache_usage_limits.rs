#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerlessCacheCacheUsageLimits {
    /// The maximum data storage limit in the cache, expressed in Gigabytes. See `data_storage` Block for details.
    #[builder(into)]
    #[serde(rename = "dataStorage")]
    pub r#data_storage: Option<Box<super::super::types::elasticache::ServerlessCacheCacheUsageLimitsDataStorage>>,
    /// The configuration for the number of ElastiCache Processing Units (ECPU) the cache can consume per second. See `ecpu_per_second` Block for details.
    #[builder(into)]
    #[serde(rename = "ecpuPerSeconds")]
    pub r#ecpu_per_seconds: Option<Vec<super::super::types::elasticache::ServerlessCacheCacheUsageLimitsEcpuPerSecond>>,
}
