#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UptimeCheckConfigTcpCheck {
    /// Contains information needed to add pings to a TCP check.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pingConfig")]
    pub r#ping_config: Option<Box<super::super::types::monitoring::UptimeCheckConfigTcpCheckPingConfig>>,
    /// The port to the page to run the check against. Will be combined with host (specified within the `monitored_resource`) to construct the full URL.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
