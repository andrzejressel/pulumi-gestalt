/// Provides a resource to associate additional IPv6 CIDR blocks with a VPC.
///
/// The `aws.ec2.VpcIpv6CidrBlockAssociation` resource allows IPv6 CIDR blocks to be added to the VPC.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = vpc::create(
///         "test",
///         VpcArgs::builder().cidr_block("10.0.0.0/16").build_struct(),
///     );
///     let testVpcIpv6CidrBlockAssociation = vpc_ipv_6_cidr_block_association::create(
///         "testVpcIpv6CidrBlockAssociation",
///         VpcIpv6CidrBlockAssociationArgs::builder()
///             .ipv_6_ipam_pool_id("${testAwsVpcIpamPool.id}")
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_vpc_ipv6_cidr_block_association` using the VPC CIDR Association ID. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpv6CidrBlockAssociation:VpcIpv6CidrBlockAssociation example vpc-cidr-assoc-xxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipv_6_cidr_block_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpv6CidrBlockAssociationArgs {
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_pam_pool_id`, `ipv6_pool`, `ipv6_cidr_block` and `ipv6_netmask_length`.
        #[builder(into, default)]
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The IPv6 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv6_netmask_length`. This parameter is required if `ipv6_netmask_length` is not set and the IPAM pool does not have `allocation_default_netmask` set. Conflicts with `assign_generated_ipv6_cidr_block`.
        #[builder(into, default)]
        pub ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// - (Optional) The ID of an IPv6 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Conflict with `assign_generated_ipv6_cidr_block` and `ipv6_ipam_pool_id`.
        #[builder(into, default)]
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The netmask length of the IPv6 CIDR you want to allocate to this VPC. Requires specifying a `ipv6_ipam_pool_id`. This parameter is optional if the IPAM pool has `allocation_default_netmask` set, otherwise it or `ipv6_cidr_block` are required. Conflicts with `assign_generated_ipv6_cidr_block` and `ipv6_ipam_pool_id`.
        #[builder(into, default)]
        pub ipv6_netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The  ID of an IPv6 address pool from which to allocate the IPv6 CIDR block. Conflicts with `ipv6_pam_pool_id`, `ipv6_pool`.
        #[builder(into, default)]
        pub ipv6_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the VPC to make the association with.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcIpv6CidrBlockAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block. Default is `false`. Conflicts with `ipv6_pam_pool_id`, `ipv6_pool`, `ipv6_cidr_block` and `ipv6_netmask_length`.
        pub assign_generated_ipv6_cidr_block: pulumi_gestalt_rust::Output<bool>,
        /// The source that allocated the IP address space. Values: `amazon`, `byoip`, `none`.
        pub ip_source: pulumi_gestalt_rust::Output<String>,
        /// Public IPv6 addresses are those advertised on the internet from AWS. Private IP addresses are not and cannot be advertised on the internet from AWS. Values: `public`, `private`.
        pub ipv6_address_attribute: pulumi_gestalt_rust::Output<String>,
        /// The IPv6 CIDR block for the VPC. CIDR can be explicitly set or it can be derived from IPAM using `ipv6_netmask_length`. This parameter is required if `ipv6_netmask_length` is not set and the IPAM pool does not have `allocation_default_netmask` set. Conflicts with `assign_generated_ipv6_cidr_block`.
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// - (Optional) The ID of an IPv6 IPAM pool you want to use for allocating this VPC's CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts. Conflict with `assign_generated_ipv6_cidr_block` and `ipv6_ipam_pool_id`.
        pub ipv6_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The netmask length of the IPv6 CIDR you want to allocate to this VPC. Requires specifying a `ipv6_ipam_pool_id`. This parameter is optional if the IPAM pool has `allocation_default_netmask` set, otherwise it or `ipv6_cidr_block` are required. Conflicts with `assign_generated_ipv6_cidr_block` and `ipv6_ipam_pool_id`.
        pub ipv6_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The  ID of an IPv6 address pool from which to allocate the IPv6 CIDR block. Conflicts with `ipv6_pam_pool_id`, `ipv6_pool`.
        pub ipv6_pool: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPC to make the association with.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpv6CidrBlockAssociationArgs,
    ) -> VpcIpv6CidrBlockAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assign_generated_ipv6_cidr_block_binding = args
            .assign_generated_ipv6_cidr_block
            .get_output(context);
        let ipv6_cidr_block_binding = args.ipv6_cidr_block.get_output(context);
        let ipv6_ipam_pool_id_binding = args.ipv6_ipam_pool_id.get_output(context);
        let ipv6_netmask_length_binding = args.ipv6_netmask_length.get_output(context);
        let ipv6_pool_binding = args.ipv6_pool.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpv6CidrBlockAssociation:VpcIpv6CidrBlockAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignGeneratedIpv6CidrBlock".into(),
                    value: &assign_generated_ipv6_cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6CidrBlock".into(),
                    value: &ipv6_cidr_block_binding.drop_type(),
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
                    name: "ipv6Pool".into(),
                    value: &ipv6_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpv6CidrBlockAssociationResult {
            id: o.get_field("id"),
            assign_generated_ipv6_cidr_block: o
                .get_field("assignGeneratedIpv6CidrBlock"),
            ip_source: o.get_field("ipSource"),
            ipv6_address_attribute: o.get_field("ipv6AddressAttribute"),
            ipv6_cidr_block: o.get_field("ipv6CidrBlock"),
            ipv6_ipam_pool_id: o.get_field("ipv6IpamPoolId"),
            ipv6_netmask_length: o.get_field("ipv6NetmaskLength"),
            ipv6_pool: o.get_field("ipv6Pool"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
