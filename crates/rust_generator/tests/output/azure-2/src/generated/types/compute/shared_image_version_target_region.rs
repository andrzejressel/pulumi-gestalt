#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SharedImageVersionTargetRegion {
    /// The ID of the Disk Encryption Set to encrypt the Image Version in the target region. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// Specifies whether this Shared Image Version should be excluded when querying for the `latest` version. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "excludeFromLatestEnabled")]
    pub r#exclude_from_latest_enabled: Option<bool>,
    /// The Azure Region in which this Image Version should exist.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The number of replicas of the Image Version to be created per region.
    #[builder(into)]
    #[serde(rename = "regionalReplicaCount")]
    pub r#regional_replica_count: i32,
    /// The storage account type for the image version. Possible values are `Standard_LRS`, `Premium_LRS` and `Standard_ZRS`. Defaults to `Standard_LRS`. You can store all of your image version replicas in Zone Redundant Storage by specifying `Standard_ZRS`.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Option<String>,
}
