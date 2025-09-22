#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinuxWebAppLogHttpLogAzureBlobStorage {
    /// The retention period in days.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: i32,
    /// The SAS url to an Azure blob container.
    #[builder(into)]
    #[serde(rename = "sasUrl")]
    pub r#sas_url: String,
}
