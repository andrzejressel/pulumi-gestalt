#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetExpressRouteCircuitServiceProviderProperty {
    /// The bandwidth in Mbps of the ExpressRoute circuit.
    #[builder(into)]
    #[serde(rename = "bandwidthInMbps")]
    pub r#bandwidth_in_mbps: i32,
    /// The name of the peering location and **not** the Azure resource location.
    #[builder(into)]
    #[serde(rename = "peeringLocation")]
    pub r#peering_location: String,
    /// The name of the ExpressRoute Service Provider.
    #[builder(into)]
    #[serde(rename = "serviceProviderName")]
    pub r#service_provider_name: String,
}
