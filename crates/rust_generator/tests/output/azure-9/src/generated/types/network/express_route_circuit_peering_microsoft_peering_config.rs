#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExpressRouteCircuitPeeringMicrosoftPeeringConfig {
    /// The communities of Bgp Peering specified for microsoft peering.
    #[builder(into)]
    #[serde(rename = "advertisedCommunities")]
    pub r#advertised_communities: Option<Vec<String>>,
    /// A list of Advertised Public Prefixes.
    #[builder(into)]
    #[serde(rename = "advertisedPublicPrefixes")]
    pub r#advertised_public_prefixes: Vec<String>,
    /// The CustomerASN of the peering. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "customerAsn")]
    pub r#customer_asn: Option<i32>,
    /// The Routing Registry against which the AS number and prefixes are registered. For example: `ARIN`, `RIPE`, `AFRINIC` etc. Defaults to `NONE`.
    #[builder(into)]
    #[serde(rename = "routingRegistryName")]
    pub r#routing_registry_name: Option<String>,
}
