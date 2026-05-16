#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentLoggingStorageAccount {
    /// Specify the container name in the Storage Account for logging.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Option<String>,
    /// The name of the StorageAccount for NGINX Logging.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
