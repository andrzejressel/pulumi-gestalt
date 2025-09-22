#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettingsSpaceStorageSettings {
    /// The default EBS storage settings for a private space. See Default EBS Storage Settings below.
    #[builder(into)]
    #[serde(rename = "defaultEbsStorageSettings")]
    pub r#default_ebs_storage_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings>>,
}
