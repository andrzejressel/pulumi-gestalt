#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciLogicalNetworkSubnet {
    /// The address prefix in CIDR notation. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Option<String>,
    /// The IP address allocation method for the subnet. Possible values are `Dynamic` and `Static`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "ipAllocationMethod")]
    pub r#ip_allocation_method: String,
    /// One or more `ip_pool` block as defined above. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** If `ip_pool` is not specified, it will be assigned by the server. If you experience a diff you may need to add this to `ignore_changes`.
    #[builder(into)]
    #[serde(rename = "ipPools")]
    pub r#ip_pools: Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetIpPool>>,
    /// A `route` block as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "routes")]
    pub r#routes: Option<Vec<super::super::types::stack::HciLogicalNetworkSubnetRoute>>,
    /// The VLAN ID for the Logical Network. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: Option<i32>,
}
