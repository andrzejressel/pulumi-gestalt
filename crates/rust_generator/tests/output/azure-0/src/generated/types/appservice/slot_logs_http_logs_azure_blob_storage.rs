#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SlotLogsHttpLogsAzureBlobStorage {
    /// The number of days to retain logs for.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: i32,
    /// The URL to the storage container, with a Service SAS token appended. **NOTE:** there is currently no means of generating Service SAS tokens with the `azurerm` provider.
    #[builder(into)]
    #[serde(rename = "sasUrl")]
    pub r#sas_url: String,
}
