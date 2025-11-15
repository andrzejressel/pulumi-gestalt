#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci {
    /// The Google Cloud Service Account Email used for auth when secretType is gcpServiceAccount
    #[builder(into)]
    #[serde(rename = "gcpServiceAccountEmail")]
    pub r#gcp_service_account_email: Option<String>,
    /// The absolute path of the directory that contains the local resources. Default: the root directory of the image
    #[builder(into)]
    #[serde(rename = "policyDir")]
    pub r#policy_dir: Option<String>,
    /// Type of secret configured for access to the Git repo
    #[builder(into)]
    #[serde(rename = "secretType")]
    pub r#secret_type: String,
    /// The OCI image repository URL for the package to sync from
    #[builder(into)]
    #[serde(rename = "syncRepo")]
    pub r#sync_repo: Option<String>,
    /// Period in seconds between consecutive syncs. Default: 15
    #[builder(into)]
    #[serde(rename = "syncWaitSecs")]
    pub r#sync_wait_secs: Option<String>,
    /// (Optional, Deprecated)
    /// Version of Config Sync installed
    /// 
    /// > **Warning:** The `configmanagement.config_sync.oci.version` field is deprecated and will be removed in a future major release. Please use `configmanagement.version` field to specify the version of Config Sync installed instead.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
