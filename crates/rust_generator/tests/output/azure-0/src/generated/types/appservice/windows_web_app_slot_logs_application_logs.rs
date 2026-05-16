#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotLogsApplicationLogs {
    /// An `azure_blob_storage` block as defined below.
    #[builder(into)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Option<Box<super::super::types::appservice::WindowsWebAppSlotLogsApplicationLogsAzureBlobStorage>>,
    /// Log level. Possible values include: `Off`, `Verbose`, `Information`, `Warning`, and `Error`.
    #[builder(into)]
    #[serde(rename = "fileSystemLevel")]
    pub r#file_system_level: String,
}
