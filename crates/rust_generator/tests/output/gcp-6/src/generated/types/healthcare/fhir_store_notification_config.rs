#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FhirStoreNotificationConfig {
    /// The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.
    /// PubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.
    /// It is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message
    /// was published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a
    /// project. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given
    /// Cloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<String>,
    /// Whether to send full FHIR resource to this Pub/Sub topic for Create and Update operation.
    /// Note that setting this to true does not guarantee that all resources will be sent in the format of
    /// full FHIR resource. When a resource change is too large or during heavy traffic, only the resource name will be
    /// sent. Clients should always check the "payloadType" label from a Pub/Sub message to determine whether
    /// it needs to fetch the full resource as a separate operation.
    #[builder(into, default)]
    #[serde(rename = "sendFullResource")]
    pub r#send_full_resource: Box<Option<bool>>,
    /// Whether to send full FHIR resource to this Pub/Sub topic for deleting FHIR resource. Note that setting this to
    /// true does not guarantee that all previous resources will be sent in the format of full FHIR resource. When a
    /// resource change is too large or during heavy traffic, only the resource name will be sent. Clients should always
    /// check the "payloadType" label from a Pub/Sub message to determine whether it needs to fetch the full previous
    /// resource as a separate operation.
    #[builder(into, default)]
    #[serde(rename = "sendPreviousResourceOnDelete")]
    pub r#send_previous_resource_on_delete: Box<Option<bool>>,
}
