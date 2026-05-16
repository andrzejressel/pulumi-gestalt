#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSlotLogsHttpLogs {
    /// A `azure_blob_storage_http` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Option<Box<super::super::types::appservice::LinuxWebAppSlotLogsHttpLogsAzureBlobStorage>>,
    /// A `file_system` block as defined above.
    #[builder(into)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Option<Box<super::super::types::appservice::LinuxWebAppSlotLogsHttpLogsFileSystem>>,
}
