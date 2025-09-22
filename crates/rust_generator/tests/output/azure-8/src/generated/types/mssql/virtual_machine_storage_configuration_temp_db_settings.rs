#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineStorageConfigurationTempDbSettings {
    /// The SQL Server default file count. This value defaults to `8`
    #[builder(into)]
    #[serde(rename = "dataFileCount")]
    pub r#data_file_count: Option<i32>,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    #[serde(rename = "dataFileGrowthInMb")]
    pub r#data_file_growth_in_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    #[serde(rename = "dataFileSizeMb")]
    pub r#data_file_size_mb: Option<i32>,
    /// The SQL Server default path
    #[builder(into)]
    #[serde(rename = "defaultFilePath")]
    pub r#default_file_path: String,
    /// The SQL Server default file size - This value defaults to `512`
    #[builder(into)]
    #[serde(rename = "logFileGrowthMb")]
    pub r#log_file_growth_mb: Option<i32>,
    /// The SQL Server default file size - This value defaults to `256`
    #[builder(into)]
    #[serde(rename = "logFileSizeMb")]
    pub r#log_file_size_mb: Option<i32>,
    /// A list of Logical Unit Numbers for the disks.
    #[builder(into)]
    #[serde(rename = "luns")]
    pub r#luns: Vec<i32>,
}
