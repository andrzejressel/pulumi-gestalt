#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsVirtualMachineScaleSetDataDisk {
    /// The type of Caching which should be used for this Data Disk. Possible values are `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// The create option which should be used for this Data Disk. Possible values are `Empty` and `FromImage`. Defaults to `Empty`. (`FromImage` should only be used if the source image includes data disks).
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: Option<String>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this Data Disk. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
    /// 
    /// > **Note:** Disk Encryption Sets are in Public Preview in a limited set of regions
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// The size of the Data Disk which should be created.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: i32,
    /// The Logical Unit Number of the Data Disk, which must be unique within the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: i32,
    /// The name of the Data Disk.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The Type of Storage Account which should back this Data Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS` and `UltraSSD_LRS`.
    /// 
    /// > **Note:** `UltraSSD_LRS` is only supported when `ultra_ssd_enabled` within the `additional_capabilities` block is enabled.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: String,
    /// Specifies the Read-Write IOPS for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into)]
    #[serde(rename = "ultraSsdDiskIopsReadWrite")]
    pub r#ultra_ssd_disk_iops_read_write: Option<i32>,
    /// Specifies the bandwidth in MB per second for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into)]
    #[serde(rename = "ultraSsdDiskMbpsReadWrite")]
    pub r#ultra_ssd_disk_mbps_read_write: Option<i32>,
    /// Should Write Accelerator be enabled for this Data Disk? Defaults to `false`.
    /// 
    /// > **Note:** This requires that the `storage_account_type` is set to `Premium_LRS` and that `caching` is set to `None`.
    #[builder(into)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Option<bool>,
}
