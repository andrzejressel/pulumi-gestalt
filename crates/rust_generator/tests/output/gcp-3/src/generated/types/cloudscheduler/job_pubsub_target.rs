#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobPubsubTarget {
    /// Attributes for PubsubMessage.
    /// Pubsub message must contain either non-empty data, or at least one attribute.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<std::collections::HashMap<String, String>>,
    /// The message payload for PubsubMessage.
    /// Pubsub message must contain either non-empty data, or at least one attribute.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: Option<String>,
    /// The full resource name for the Cloud Pub/Sub topic to which
    /// messages will be published when a job is delivered. ~>**NOTE:**
    /// The topic name must be in the same format as required by PubSub's
    /// PublishRequest.name, e.g. `projects/my-project/topics/my-topic`.
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: String,
}
