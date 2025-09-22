#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVolumeGroupSapHanaVolumeDataProtectionReplication {
    /// The endpoint type.
    #[builder(into)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: String,
    /// Location of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeLocation")]
    pub r#remote_volume_location: String,
    /// Resource ID of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeResourceId")]
    pub r#remote_volume_resource_id: String,
    /// Replication frequency.
    #[builder(into)]
    #[serde(rename = "replicationFrequency")]
    pub r#replication_frequency: String,
}
