#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcArgs {
        /// Cidr block of the desired VPC.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean constraint on whether the desired VPC is
        /// the default VPC for the region.
        #[builder(into, default)]
        pub default: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// DHCP options id of the desired VPC.
        #[builder(into, default)]
        pub dhcp_options_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcFilter>>,
        >,
        /// ID of the specific VPC to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Current state of the desired VPC.
        /// Can be either `"pending"` or `"available"`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired VPC.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcResult {
        /// ARN of VPC
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// CIDR block for the association.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        pub cidr_block_associations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetVpcCidrBlockAssociation>,
        >,
        pub default: pulumi_gestalt_rust::Output<bool>,
        pub dhcp_options_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the VPC has DNS hostname support
        pub enable_dns_hostnames: pulumi_gestalt_rust::Output<bool>,
        /// Whether or not the VPC has DNS support
        pub enable_dns_support: pulumi_gestalt_rust::Output<bool>,
        /// Whether Network Address Usage metrics are enabled for your VPC
        pub enable_network_address_usage_metrics: pulumi_gestalt_rust::Output<bool>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Allowed tenancy of instances launched into the
        /// selected VPC. May be any of `"default"`, `"dedicated"`, or `"host"`.
        pub instance_tenancy: pulumi_gestalt_rust::Output<String>,
        /// Association ID for the IPv6 CIDR block.
        pub ipv6_association_id: pulumi_gestalt_rust::Output<String>,
        /// IPv6 CIDR block.
        pub ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// ID of the main route table associated with this VPC.
        pub main_route_table_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns the VPC.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// State of the association.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcArgs,
    ) -> GetVpcResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_block_binding = args.cidr_block.get_output(context);
        let default_binding = args.default.get_output(context);
        let dhcp_options_id_binding = args.dhcp_options_id.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let state_binding = args.state.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpc:getVpc".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "default".into(),
                    value: &default_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dhcpOptionsId".into(),
                    value: &dhcp_options_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcResult {
            arn: o.get_field("arn"),
            cidr_block: o.get_field("cidrBlock"),
            cidr_block_associations: o.get_field("cidrBlockAssociations"),
            default: o.get_field("default"),
            dhcp_options_id: o.get_field("dhcpOptionsId"),
            enable_dns_hostnames: o.get_field("enableDnsHostnames"),
            enable_dns_support: o.get_field("enableDnsSupport"),
            enable_network_address_usage_metrics: o
                .get_field("enableNetworkAddressUsageMetrics"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            instance_tenancy: o.get_field("instanceTenancy"),
            ipv6_association_id: o.get_field("ipv6AssociationId"),
            ipv6_cidr_block: o.get_field("ipv6CidrBlock"),
            main_route_table_id: o.get_field("mainRouteTableId"),
            owner_id: o.get_field("ownerId"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
        }
    }
}
