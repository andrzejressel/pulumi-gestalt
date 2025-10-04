/// Previews a CIDR from an IPAM address pool. Only works for private IPv4.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamPreviewNextCidr
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       netmaskLength: 28
///       disallowedCidrs:
///         - 172.2.0.0/32
///     options:
///       dependsOn:
///         - ${exampleVpcIpamPoolCidr}
///   exampleVpcIpamPoolCidr:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: example
///     properties:
///       ipamPoolId: ${exampleVpcIpamPool.id}
///       cidr: 172.20.0.0/16
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${exampleVpcIpam.privateDefaultScopeId}
///       locale: ${current.name}
///   exampleVpcIpam:
///     type: aws:ec2:VpcIpam
///     name: example
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_preview_next_cidr {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrArgs {
        /// Exclude a particular CIDR range from being returned by the pool.
        #[builder(into, default)]
        pub disallowed_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        #[builder(into)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        #[builder(into, default)]
        pub netmask_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPreviewNextCidrResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The previewed CIDR from the pool.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// Exclude a particular CIDR range from being returned by the pool.
        pub disallowed_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the pool to which you want to assign a CIDR.
        pub ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The netmask length of the CIDR you would like to preview from the IPAM pool.
        pub netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamPreviewNextCidrArgs,
    ) -> VpcIpamPreviewNextCidrResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let disallowed_cidrs_binding = args.disallowed_cidrs.get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let netmask_length_binding = args.netmask_length.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPreviewNextCidr:VpcIpamPreviewNextCidr".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disallowedCidrs".into(),
                    value: &disallowed_cidrs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netmaskLength".into(),
                    value: &netmask_length_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamPreviewNextCidrResult {
            id: o.get_field("id"),
            cidr: o.get_field("cidr"),
            disallowed_cidrs: o.get_field("disallowedCidrs"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            netmask_length: o.get_field("netmaskLength"),
        }
    }
}
