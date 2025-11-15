#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FileCacheLustreConfiguration {
    /// Specifies the cache deployment type. The only supported value is `CACHE_1`.
    #[builder(into)]
    #[serde(rename = "deploymentType")]
    pub r#deployment_type: String,
    #[builder(into)]
    #[serde(rename = "logConfigurations")]
    pub r#log_configurations: Option<Vec<super::super::types::fsx::FileCacheLustreConfigurationLogConfiguration>>,
    /// The configuration for a Lustre MDT (Metadata Target) storage volume. See the `metadata_configuration` block.
    #[builder(into)]
    #[serde(rename = "metadataConfigurations")]
    pub r#metadata_configurations: Vec<super::super::types::fsx::FileCacheLustreConfigurationMetadataConfiguration>,
    #[builder(into)]
    #[serde(rename = "mountName")]
    pub r#mount_name: Option<String>,
    /// Provisions the amount of read and write throughput for each 1 tebibyte (TiB) of cache storage capacity, in MB/s/TiB. The only supported value is `1000`.
    #[builder(into)]
    #[serde(rename = "perUnitStorageThroughput")]
    pub r#per_unit_storage_throughput: i32,
    /// A recurring weekly time, in the format `D:HH:MM`. `D` is the day of the week, for which `1` represents Monday and `7` represents Sunday. `HH` is the zero-padded hour of the day (0-23), and `MM` is the zero-padded minute of the hour. For example, 1:05:00 specifies maintenance at 5 AM Monday. See the [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date) for more information.
    #[builder(into)]
    #[serde(rename = "weeklyMaintenanceStartTime")]
    pub r#weekly_maintenance_start_time: Option<String>,
}
