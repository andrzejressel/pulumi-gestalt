#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
    /// The default size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "defaultEbsVolumeSizeInGb")]
    pub r#default_ebs_volume_size_in_gb: i32,
    /// The maximum size of the EBS storage volume for a private space.
    #[builder(into)]
    #[serde(rename = "maximumEbsVolumeSizeInGb")]
    pub r#maximum_ebs_volume_size_in_gb: i32,
}
