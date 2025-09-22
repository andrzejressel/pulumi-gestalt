#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetExadataInfrastructureEstimatedPatchingTime {
    /// The estimated time required in minutes for database server patching.
    #[builder(into)]
    #[serde(rename = "estimatedDbServerPatchingTime")]
    pub r#estimated_db_server_patching_time: i32,
    /// The estimated time required in minutes for network switch patching.
    #[builder(into)]
    #[serde(rename = "estimatedNetworkSwitchesPatchingTime")]
    pub r#estimated_network_switches_patching_time: i32,
    /// The estimated time required in minutes for storage server patching.
    #[builder(into)]
    #[serde(rename = "estimatedStorageServerPatchingTime")]
    pub r#estimated_storage_server_patching_time: i32,
    /// The estimated total time required in minutes for all patching operations.
    #[builder(into)]
    #[serde(rename = "totalEstimatedPatchingTime")]
    pub r#total_estimated_patching_time: i32,
}
