#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerPoolOrigin {
    /// The IP address (IPv4 or IPv6) of the origin, or the publicly addressable hostname.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Whether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// HTTP request headers.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::types::LoadBalancerPoolOriginHeader>>>,
    /// A human-identifiable name for the origin.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The virtual network subnet ID the origin belongs in. Virtual network must also belong to the account.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<Option<String>>,
    /// The weight (0.01 - 1.00) of this origin, relative to other origins in the pool. Equal values mean equal weighting. A weight of 0 means traffic will not be sent to this origin, but health is still checked. When `origin_steering.policy="least_outstanding_requests"`, weight is used to scale the origin's outstanding requests. When `origin_steering.policy="least_connections"`, weight is used to scale the origin's open connections. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<f64>>,
}
