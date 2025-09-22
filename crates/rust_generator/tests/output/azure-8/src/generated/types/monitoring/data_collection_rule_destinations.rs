#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataCollectionRuleDestinations {
    /// A `azure_monitor_metrics` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureMonitorMetrics")]
    pub r#azure_monitor_metrics: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsAzureMonitorMetrics>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "eventHub")]
    pub r#event_hub: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsEventHub>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "eventHubDirect")]
    pub r#event_hub_direct: Box<Option<super::super::types::monitoring::DataCollectionRuleDestinationsEventHubDirect>>,
    /// One or more `log_analytics` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "logAnalytics")]
    pub r#log_analytics: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsLogAnalytic>>,
    /// One or more `monitor_account` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "monitorAccounts")]
    pub r#monitor_accounts: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsMonitorAccount>>,
    /// One or more `storage_blob_direct` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobDirects")]
    pub r#storage_blob_directs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlobDirect>>,
    /// One or more `storage_blob` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobs")]
    pub r#storage_blobs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlob>>,
    /// One or more `storage_table_direct` blocks as defined below.
    /// 
    /// > **NOTE** `event_hub_direct`, `storage_blob_direct`, and `storage_table_direct` are only available for rules of kind `AgentDirectToStore`.
    /// 
    /// > **NOTE** At least one of `azure_monitor_metrics`, `event_hub`, `event_hub_direct`, `log_analytics`, `monitor_account`, `storage_blob`, `storage_blob_direct`,and `storage_table_direct` blocks must be specified.
    #[builder(into)]
    #[serde(rename = "storageTableDirects")]
    pub r#storage_table_directs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageTableDirect>>,
}
