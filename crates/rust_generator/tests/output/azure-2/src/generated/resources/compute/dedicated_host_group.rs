/// Manage a Dedicated Host Group.
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
///             .name("example-rg-compute")
///             .build_struct(),
///     );
///     let exampleDedicatedHostGroup = dedicated_host_group::create(
///         "exampleDedicatedHostGroup",
///         DedicatedHostGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-dedicated-host-group")
///             .platform_fault_domain_count(1)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Dedicated Host Group can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/dedicatedHostGroup:DedicatedHostGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Compute/hostGroups/group1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod dedicated_host_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedHostGroupArgs {
        /// Would virtual machines or virtual machine scale sets be placed automatically on this Dedicated Host Group? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub automatic_placement_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The Azure location where the Dedicated Host Group exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name of the Dedicated Host Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The number of fault domains that the Dedicated Host Group spans. Changing this forces a new resource to be created.
        #[builder(into)]
        pub platform_fault_domain_count: pulumi_gestalt_rust::Input<i32>,
        /// Specifies the name of the resource group the Dedicated Host Group is located in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone in which this Dedicated Host Group should be located. Changing this forces a new Dedicated Host Group to be created.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DedicatedHostGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Would virtual machines or virtual machine scale sets be placed automatically on this Dedicated Host Group? Defaults to `false`. Changing this forces a new resource to be created.
        pub automatic_placement_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Azure location where the Dedicated Host Group exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Dedicated Host Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of fault domains that the Dedicated Host Group spans. Changing this forces a new resource to be created.
        pub platform_fault_domain_count: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the resource group the Dedicated Host Group is located in. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone in which this Dedicated Host Group should be located. Changing this forces a new Dedicated Host Group to be created.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedHostGroupArgs,
    ) -> DedicatedHostGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedHostGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DedicatedHostGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedHostGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DedicatedHostGroupResult {
        let automatic_placement_enabled_binding = args
            .automatic_placement_enabled
            .get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let platform_fault_domain_count_binding = args
            .platform_fault_domain_count
            .get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let zone_binding = args.zone.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/dedicatedHostGroup:DedicatedHostGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticPlacementEnabled".into(),
                    value: &automatic_placement_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformFaultDomainCount".into(),
                    value: &platform_fault_domain_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DedicatedHostGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            automatic_placement_enabled: o.get_field("automaticPlacementEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            platform_fault_domain_count: o.get_field("platformFaultDomainCount"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            zone: o.get_field("zone"),
        }
    }
}
