#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSharedImageVersionsImageTargetRegion {
    /// The Azure Region in which this Image Version exists.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The number of replicas of the Image Version to be created per region.
    #[builder(into)]
    #[serde(rename = "regionalReplicaCount")]
    pub r#regional_replica_count: i32,
    /// The storage account type for the image version.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: String,
}
