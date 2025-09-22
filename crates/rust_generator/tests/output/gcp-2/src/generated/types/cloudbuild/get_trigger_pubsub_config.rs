#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTriggerPubsubConfig {
    /// Service account that will make the push request.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: String,
    /// Potential issues with the underlying Pub/Sub subscription configuration.
    /// Only populated on get requests.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// Output only. Name of the subscription.
    #[builder(into)]
    #[serde(rename = "subscription")]
    pub r#subscription: String,
    /// The name of the topic from which this subscription is receiving messages.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}
