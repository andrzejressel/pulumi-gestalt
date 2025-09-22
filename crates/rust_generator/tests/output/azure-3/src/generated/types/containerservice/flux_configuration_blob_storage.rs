#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FluxConfigurationBlobStorage {
    /// Specifies the account key (shared key) to access the storage account.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Option<String>,
    /// Specifies the Azure Blob container ID.
    #[builder(into)]
    #[serde(rename = "containerId")]
    pub r#container_id: String,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets.
    #[builder(into)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Option<String>,
    /// A `managed_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "managedIdentity")]
    pub r#managed_identity: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageManagedIdentity>>,
    /// Specifies the shared access token to access the storage container.
    #[builder(into)]
    #[serde(rename = "sasToken")]
    pub r#sas_token: Option<String>,
    /// A `service_principal` block as defined below.
    #[builder(into)]
    #[serde(rename = "servicePrincipal")]
    pub r#service_principal: Option<Box<super::super::types::containerservice::FluxConfigurationBlobStorageServicePrincipal>>,
    /// Specifies the interval at which to re-reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster Azure Blob source with the remote.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
}
