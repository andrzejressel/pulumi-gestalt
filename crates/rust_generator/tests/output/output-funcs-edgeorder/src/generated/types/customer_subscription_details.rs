#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomerSubscriptionDetails {
    /// Location placement Id of a subscription
    #[builder(into)]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: Option<String>,
    /// Quota ID of a subscription
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: String,
    /// List of registered feature flags for subscription
    #[builder(into)]
    #[serde(rename = "registeredFeatures")]
    pub r#registered_features: Option<Vec<super::types::CustomerSubscriptionRegisteredFeatures>>,
}
