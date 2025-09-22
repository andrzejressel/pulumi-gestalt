#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigActionPubSubNotification {
    /// How much data to include in the pub/sub message.
    /// Possible values are: `TABLE_PROFILE`, `RESOURCE_NAME`.
    #[builder(into)]
    #[serde(rename = "detailOfMessage")]
    pub r#detail_of_message: Option<String>,
    /// The type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted.
    /// Possible values are: `NEW_PROFILE`, `CHANGED_PROFILE`, `SCORE_INCREASED`, `ERROR_CHANGED`.
    #[builder(into)]
    #[serde(rename = "event")]
    pub r#event: Option<String>,
    /// Conditions for triggering pubsub
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubsubCondition")]
    pub r#pubsub_condition: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotificationPubsubCondition>>,
    /// Cloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Option<String>,
}
