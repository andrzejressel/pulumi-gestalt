#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyInsightsLogAnalyticsWorkspace {
    /// The location of the Firewalls, that when matches this Log Analytics Workspace will be used to consume their logs.
    #[builder(into)]
    #[serde(rename = "firewallLocation")]
    pub r#firewall_location: String,
    /// The ID of the Log Analytics Workspace that the Firewalls associated with this Firewall Policy will send their logs to when their locations match the `firewall_location`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
