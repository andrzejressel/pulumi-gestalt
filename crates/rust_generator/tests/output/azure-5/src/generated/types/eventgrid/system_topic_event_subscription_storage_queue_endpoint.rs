#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SystemTopicEventSubscriptionStorageQueueEndpoint {
    /// Storage queue message time to live in seconds.
    #[builder(into)]
    #[serde(rename = "queueMessageTimeToLiveInSeconds")]
    pub r#queue_message_time_to_live_in_seconds: Option<i32>,
    /// Specifies the name of the storage queue where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: String,
    /// Specifies the id of the storage account id where the storage queue is located.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: String,
}
