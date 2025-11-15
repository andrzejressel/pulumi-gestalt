#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisProxy {
    /// The path in the data store to be used when moving data between Self-Hosted and Azure-SSIS Integration Runtimes.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Name of Self Hosted Integration Runtime as a proxy.
    #[builder(into)]
    #[serde(rename = "selfHostedIntegrationRuntimeName")]
    pub r#self_hosted_integration_runtime_name: String,
    /// Name of Azure Blob Storage linked service to reference the staging data store to be used when moving data between self-hosted and Azure-SSIS integration runtimes.
    #[builder(into)]
    #[serde(rename = "stagingStorageLinkedServiceName")]
    pub r#staging_storage_linked_service_name: String,
}
