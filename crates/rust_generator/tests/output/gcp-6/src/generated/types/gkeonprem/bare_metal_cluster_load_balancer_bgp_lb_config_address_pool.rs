#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalClusterLoadBalancerBgpLbConfigAddressPool {
    /// The addresses that are part of this pool. Each address must be either in the CIDR form (1.2.3.0/24) or range form (1.2.3.1-1.2.3.5).
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Vec<String>,
    /// If true, avoid using IPs ending in .0 or .255.
    /// This avoids buggy consumer devices mistakenly dropping IPv4 traffic for those special IP addresses.
    #[builder(into)]
    #[serde(rename = "avoidBuggyIps")]
    pub r#avoid_buggy_ips: Option<bool>,
    /// If true, prevent IP addresses from being automatically assigned.
    #[builder(into)]
    #[serde(rename = "manualAssign")]
    pub r#manual_assign: Option<String>,
    /// The name of the address pool.
    #[builder(into)]
    #[serde(rename = "pool")]
    pub r#pool: String,
}
