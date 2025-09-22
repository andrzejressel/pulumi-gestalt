#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DefaultNetworkAclEgress {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The CIDR block to match. This must be a valid network mask.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Option<String>,
    /// The from port to match.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// The ICMP type code to be used. Default 0.
    #[builder(into)]
    #[serde(rename = "icmpCode")]
    pub r#icmp_code: Option<i32>,
    /// The ICMP type to be used. Default 0.
    #[builder(into)]
    #[serde(rename = "icmpType")]
    pub r#icmp_type: Option<i32>,
    /// The IPv6 CIDR block.
    /// 
    /// > For more information on ICMP types and codes, see [Internet Control Message Protocol (ICMP) Parameters](https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml).
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Option<String>,
    /// The protocol to match. If using the -1 'all' protocol, you must specify a from and to port of 0.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The rule number. Used for ordering.
    #[builder(into)]
    #[serde(rename = "ruleNo")]
    pub r#rule_no: i32,
    /// The to port to match.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}
