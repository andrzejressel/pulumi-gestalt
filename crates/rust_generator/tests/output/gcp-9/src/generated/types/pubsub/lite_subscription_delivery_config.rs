#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LiteSubscriptionDeliveryConfig {
    /// When this subscription should send messages to subscribers relative to messages persistence in storage.
    /// Possible values are: `DELIVER_IMMEDIATELY`, `DELIVER_AFTER_STORED`, `DELIVERY_REQUIREMENT_UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "deliveryRequirement")]
    pub r#delivery_requirement: String,
}
