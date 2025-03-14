/// Manages IP Group CIDR records.
///
/// > Warning Do not use this resource at the same time as the `cidrs` property of the
/// `azure.network.IPGroup` resource for the same IP Group. Doing so will cause a conflict and
/// CIDRS will be removed.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("test-rg")
///             .build_struct(),
///     );
///     let exampleIPGroup = ip_group::create(
///         "exampleIPGroup",
///         IpGroupArgs::builder()
///             .location("${example.location}")
///             .name("test-ipgroup")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleIPGroupCIDR = ip_group_cidr::create(
///         "exampleIPGroupCIDR",
///         IpGroupCidrArgs::builder()
///             .cidr("10.10.10.0/24")
///             .ip_group_id("${exampleIPGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IP Group CIDRs can be imported using the `resource id` of the IP Group and
///
/// the CIDR value (`/` characters have to be replaced by `_`), e.g.
///
/// ```sh
/// $ pulumi import azure:network/iPGroupCIDR:IPGroupCIDR example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/test-rg/providers/Microsoft.Network/ipGroups/test-ipgroup/cidrs/10.1.0.0_24
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ip_group_cidr {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IPGroupCIDRArgs {
        #[builder(into)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the destination IP Group.
        /// Changing this forces a new IP Group CIDR to be created.
        #[builder(into)]
        pub ip_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IPGroupCIDRResult {
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// The ID of the destination IP Group.
        /// Changing this forces a new IP Group CIDR to be created.
        pub ip_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IPGroupCIDRArgs,
    ) -> IPGroupCIDRResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_binding = args.cidr.get_output(context);
        let ip_group_id_binding = args.ip_group_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/iPGroupCIDR:IPGroupCIDR".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipGroupId".into(),
                    value: &ip_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IPGroupCIDRResult {
            cidr: o.get_field("cidr"),
            ip_group_id: o.get_field("ipGroupId"),
        }
    }
}
