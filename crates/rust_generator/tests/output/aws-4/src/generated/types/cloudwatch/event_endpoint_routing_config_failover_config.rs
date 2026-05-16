#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventEndpointRoutingConfigFailoverConfig {
    /// Parameters used for the primary Region. Documented below.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigPrimary>,
    /// Parameters used for the secondary Region, the Region that events are routed to when failover is triggered or event replication is enabled. Documented below.
    #[builder(into)]
    #[serde(rename = "secondary")]
    pub r#secondary: Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigSecondary>,
}
