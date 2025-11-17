#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetStorageProfileDataDisk {
    /// Specifies the caching requirements. Possible values include: `None` (default), `ReadOnly`, `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Option<String>,
    /// Specifies how the data disk should be created. The only possible options are `FromImage` and `Empty`.
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: String,
    /// Specifies the size of the disk in GB. This element is required when creating an empty disk.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Option<i32>,
    /// Specifies the Logical Unit Number of the disk in each virtual machine in the scale set.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: i32,
    /// Specifies the type of managed disk to create. Value must be either `Standard_LRS`, `StandardSSD_LRS` or `Premium_LRS`.
    #[builder(into)]
    #[serde(rename = "managedDiskType")]
    pub r#managed_disk_type: Option<String>,
}
