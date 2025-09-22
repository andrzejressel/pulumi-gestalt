#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityGroupIngress {
    /// List of CIDR blocks.
    #[builder(into)]
    #[serde(rename = "cidrBlocks")]
    pub r#cidr_blocks: Option<Vec<String>>,
    /// Description of this ingress rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Start port (or ICMP type number if protocol is `icmp` or `icmpv6`).
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// List of IPv6 CIDR blocks.
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlocks")]
    pub r#ipv_6_cidr_blocks: Option<Vec<String>>,
    /// List of Prefix List IDs.
    #[builder(into)]
    #[serde(rename = "prefixListIds")]
    pub r#prefix_list_ids: Option<Vec<String>>,
    /// Protocol. If you select a protocol of `-1` (semantically equivalent to `all`, which is not a valid value here), you must specify a `from_port` and `to_port` equal to 0. The supported values are defined in the `IpProtocol` argument on the [IpPermission](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_IpPermission.html) API reference.
    /// 
    /// The following arguments are optional:
    /// 
    /// > **Note** Although `cidr_blocks`, `ipv6_cidr_blocks`, `prefix_list_ids`, and `security_groups` are all marked as optional, you _must_ provide one of them in order to configure the source of the traffic.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// List of security groups. A group name can be used relative to the default VPC. Otherwise, group ID.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// Whether the security group itself will be added as a source to this ingress rule.
    #[builder(into)]
    #[serde(rename = "self")]
    pub r#self_: Option<bool>,
    /// End range port (or ICMP code if protocol is `icmp`).
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}
