/// Provides a VPC resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = vpc::create(
///         "main",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// Basic usage with tags:
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///       instanceTenancy: default
///       tags:
///         Name: main
/// ```
///
/// VPC with CIDR from AWS IPAM:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   testVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: test
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${test.privateDefaultScopeId}
///       locale: ${current.name}
///   testVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: test
///     properties:
///       ipamPoolId: ${testVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   testVpc:
///     type: aws:ec2:Vpc
///     name: test
///     properties:
///       ipv4IpamPoolId: ${testVpcIpamPool.id}
///       ipv4NetmaskLength: 28
///     options:
///       dependsOn:
///         - ${testVpcIpamPoolCidr}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPCs using the VPC `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpc:Vpc test_vpc vpc-a01106c2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcArgs {
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_ipam_pool_id`
        #[builder(into, default)]
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean flag to enable/disable DNS hostnames in the VPC. Defaults false.
        #[builder(into, default)]
        pub enable_dns_hostnames: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A boolean flag to enable/disable DNS support in the VPC. Defaults to true.
        #[builder(into, default)]
        pub enable_dns_support: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates whether Network Address Usage metrics are enabled for your VPC. Defaults to false.
        #[builder(into, default)]
        pub enable_network_address_usage_metrics: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A tenancy option for instances launched into the VPC. Default is `default`, which ensures that EC2 instances launched in this VPC use the EC2 instance tenancy attribute specified when the EC2 instance is launched. The only other option is `dedicated`, which ensures that EC2 instances launched in this VPC are run on dedicated tenancy instances regardless of the tenancy attribute specified at launch. This has a dedicated per region fee of $2 per hour, plus an hourly per instance usage fee.
        #[builder(into, default)]
        pub instance_tenancy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        #[builder(into, default)]
        pub ipv4_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        #[builder(into, default)]
        pub ipv4_netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// IPv6 CIDR block to request from an IPAM Pool. Can be set explicitly or derived from IPAM using `ipv6_netmask_length`.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// By default when an IPv6 CIDR is assigned to a VPC a default ipv6_cidr_block_network_border_group will be set to the region of the VPC. This can be changed to restrict advertisement of public addresses to specific Network Border Groups such as LocalZones.
        #[builder(into, default)]
        pub ipv6_cidr_block_network_border_group: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// IPAM Pool ID for a IPv6 pool. Conflicts with `assign_generated_ipv6_cidr_block`.
        #[builder(into, default)]
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Netmask length to request from IPAM Pool. Conflicts with `ipv6_cidr_block`. This can be omitted if IPAM pool as a `allocation_default_netmask_length` set. Valid values are from `44` to `60` in increments of 4.
        #[builder(into, default)]
        pub ipv6_netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of VPC
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_ipam_pool_id`
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The IPv4 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv4_netmask_length`.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network ACL created by default on VPC creation
        pub default_network_acl_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the route table created by default on VPC creation
        pub default_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group created by default on VPC creation
        pub default_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// DHCP options id of the desired VPC.
        pub dhcp_options_id: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag to enable/disable DNS hostnames in the VPC. Defaults false.
        pub enable_dns_hostnames: pulumi_gestalt_rust::Output<bool>,
        /// A boolean flag to enable/disable DNS support in the VPC. Defaults to true.
        pub enable_dns_support: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Indicates whether Network Address Usage metrics are enabled for your VPC. Defaults to false.
        pub enable_network_address_usage_metrics: pulumi_gestalt_rust::Output<bool>,
        /// A tenancy option for instances launched into the VPC. Default is `default`, which ensures that EC2 instances launched in this VPC use the EC2 instance tenancy attribute specified when the EC2 instance is launched. The only other option is `dedicated`, which ensures that EC2 instances launched in this VPC are run on dedicated tenancy instances regardless of the tenancy attribute specified at launch. This has a dedicated per region fee of $2 per hour, plus an hourly per instance usage fee.
        pub instance_tenancy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Using IPAM you can monitor IP address usage throughout your AWS Organization.
        pub ipv4_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The netmask length of the IPv4 CIDR you want to allocate to this VPC. Requires specifying a `ipv4_ipam_pool_id`.
        pub ipv4_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The association ID for the IPv6 CIDR block.
        pub ipv6_association_id: pulumi_gestalt_rust::Output<String>,
        /// IPv6 CIDR block to request from an IPAM Pool. Can be set explicitly or derived from IPAM using `ipv6_netmask_length`.
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// By default when an IPv6 CIDR is assigned to a VPC a default ipv6_cidr_block_network_border_group will be set to the region of the VPC. This can be changed to restrict advertisement of public addresses to specific Network Border Groups such as LocalZones.
        pub ipv6_cidr_block_network_border_group: pulumi_gestalt_rust::Output<String>,
        /// IPAM Pool ID for a IPv6 pool. Conflicts with `assign_generated_ipv6_cidr_block`.
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Netmask length to request from IPAM Pool. Conflicts with `ipv6_cidr_block`. This can be omitted if IPAM pool as a `allocation_default_netmask_length` set. Valid values are from `44` to `60` in increments of 4.
        pub ipv6_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the main route table associated with
        /// this VPC. Note that you can change a VPC's main route table by using an
        /// `aws.ec2.MainRouteTableAssociation`.
        pub main_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the AWS account that owns the VPC.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcArgs,
    ) -> VpcResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assign_generated_ipv6_cidr_block_binding = args
            .assign_generated_ipv6_cidr_block
            .get_output(context);
        let cidr_block_binding = args.cidr_block.get_output(context);
        let enable_dns_hostnames_binding = args.enable_dns_hostnames.get_output(context);
        let enable_dns_support_binding = args.enable_dns_support.get_output(context);
        let enable_network_address_usage_metrics_binding = args
            .enable_network_address_usage_metrics
            .get_output(context);
        let instance_tenancy_binding = args.instance_tenancy.get_output(context);
        let ipv4_ipam_pool_id_binding = args.ipv4_ipam_pool_id.get_output(context);
        let ipv4_netmask_length_binding = args.ipv4_netmask_length.get_output(context);
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_output(context);
        let ipv6_cidr_block_network_border_group_binding = args
            .ipv6_cidr_block_network_border_group
            .get_output(context);
        let ipv6_ipam_pool_id_binding = args.ipv6_ipam_pool_id.get_output(context);
        let ipv6_netmask_length_binding = args.ipv6_netmask_length.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpc:Vpc".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignGeneratedIpv6CidrBlock".into(),
                    value: &assign_generated_ipv6_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDnsHostnames".into(),
                    value: &enable_dns_hostnames_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDnsSupport".into(),
                    value: &enable_dns_support_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableNetworkAddressUsageMetrics".into(),
                    value: &enable_network_address_usage_metrics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceTenancy".into(),
                    value: &instance_tenancy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv4IpamPoolId".into(),
                    value: &ipv4_ipam_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv4NetmaskLength".into(),
                    value: &ipv4_netmask_length_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlockNetworkBorderGroup".into(),
                    value: &ipv6_cidr_block_network_border_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6IpamPoolId".into(),
                    value: &ipv6_ipam_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6NetmaskLength".into(),
                    value: &ipv6_netmask_length_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            assign_generated_ipv6_cidr_block: o
                .get_field("assignGeneratedIpv6CidrBlock"),
            cidr_block: o.get_field("cidrBlock"),
            default_network_acl_id: o.get_field("defaultNetworkAclId"),
            default_route_table_id: o.get_field("defaultRouteTableId"),
            default_security_group_id: o.get_field("defaultSecurityGroupId"),
            dhcp_options_id: o.get_field("dhcpOptionsId"),
            enable_dns_hostnames: o.get_field("enableDnsHostnames"),
            enable_dns_support: o.get_field("enableDnsSupport"),
            enable_network_address_usage_metrics: o
                .get_field("enableNetworkAddressUsageMetrics"),
            instance_tenancy: o.get_field("instanceTenancy"),
            ipv4_ipam_pool_id: o.get_field("ipv4IpamPoolId"),
            ipv4_netmask_length: o.get_field("ipv4NetmaskLength"),
            ipv6_association_id: o.get_field("ipv6AssociationId"),
            ipv6_cidr_block: o.get_field("ipv6CidrBlock"),
            ipv6_cidr_block_network_border_group: o
                .get_field("ipv6CidrBlockNetworkBorderGroup"),
            ipv6_ipam_pool_id: o.get_field("ipv6IpamPoolId"),
            ipv6_netmask_length: o.get_field("ipv6NetmaskLength"),
            main_route_table_id: o.get_field("mainRouteTableId"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
