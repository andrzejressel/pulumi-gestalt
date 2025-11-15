#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SlotLogsApplicationLogs {
    /// An `azure_blob_storage` block as defined below.
    #[builder(into)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Option<Box<super::super::types::appservice::SlotLogsApplicationLogsAzureBlobStorage>>,
    /// The file system log level. Possible values are `Off`, `Error`, `Warning`, `Information`, and `Verbose`. Defaults to `Off`.
    #[builder(into)]
    #[serde(rename = "fileSystemLevel")]
    pub r#file_system_level: Option<String>,
}
