#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContactListTopic {
    /// Default subscription status to be applied to a contact if the contact has not noted their preference for subscribing to a topic.
    #[builder(into)]
    #[serde(rename = "defaultSubscriptionStatus")]
    pub r#default_subscription_status: String,
    /// Description of what the topic is about, which the contact will see.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Name of the topic the contact will see.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// Name of the topic.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "topicName")]
    pub r#topic_name: String,
}
