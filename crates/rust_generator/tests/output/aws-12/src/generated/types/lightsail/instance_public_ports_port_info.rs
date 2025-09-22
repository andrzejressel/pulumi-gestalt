#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstancePublicPortsPortInfo {
    /// Set of CIDR aliases that define access for a preconfigured range of IP addresses.
    #[builder(into)]
    #[serde(rename = "cidrListAliases")]
    pub r#cidr_list_aliases: Option<Vec<String>>,
    /// Set of CIDR blocks.
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Option<Vec<String>>,
    /// First port in a range of open ports on an instance.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    #[builder(into)]
    #[serde(rename = "ipv6Cidrs")]
    pub r#ipv_6_cidrs: Option<Vec<String>>,
    /// IP protocol name. Valid values are `tcp`, `all`, `udp`, and `icmp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Last port in a range of open ports on an instance.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}
