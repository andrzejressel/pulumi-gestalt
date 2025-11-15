#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDestinationsStorageBlobDirect {
    /// The Storage Container name.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: String,
    /// The name which should be used for this destination. This name should be unique across all destinations regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The resource ID of the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: String,
}
