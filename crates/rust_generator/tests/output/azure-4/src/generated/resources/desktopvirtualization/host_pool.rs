/// Manages a Virtual Desktop Host Pool.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleHostPool = host_pool::create(
///         "exampleHostPool",
///         HostPoolArgs::builder()
///             .custom_rdp_properties("audiocapturemode:i:1;audiomode:i:0;")
///             .description("Acceptance Test: A pooled host pool - pooleddepthfirst")
///             .friendly_name("pooleddepthfirst")
///             .load_balancer_type("DepthFirst")
///             .location("${example.location}")
///             .maximum_sessions_allowed(50)
///             .name("pooleddepthfirst")
///             .resource_group_name("${example.name}")
///             .scheduled_agent_updates(
///                 HostPoolScheduledAgentUpdates::builder()
///                     .enabled(true)
///                     .schedules(
///                         vec![
///                             HostPoolScheduledAgentUpdatesSchedule::builder()
///                             .dayOfWeek("Saturday").hourOfDay(2).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .start_vm_on_connect(true)
///             .type_("Pooled")
///             .validate_environment(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Desktop Host Pools can be imported using the `resource id`, e.g.
///
/// text
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/hostPool:HostPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/hostPools/myhostpool
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod host_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostPoolArgs {
        /// A valid custom RDP properties string for the Virtual Desktop Host Pool, available properties can be [found in this article](https://docs.microsoft.com/windows-server/remote/remote-desktop-services/clients/rdp-files).
        #[builder(into, default)]
        pub custom_rdp_properties: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description for the Virtual Desktop Host Pool.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A friendly name for the Virtual Desktop Host Pool.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// `BreadthFirst` load balancing distributes new user sessions across all available session hosts in the host pool. Possible values are `BreadthFirst`, `DepthFirst` and `Persistent`.
        /// `DepthFirst` load balancing distributes new user sessions to an available session host with the highest number of connections but has not reached its maximum session limit threshold.
        /// `Persistent` should be used if the host pool type is `Personal`
        #[builder(into)]
        pub load_balancer_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location/region where the Virtual Desktop Host Pool is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valid integer value from 0 to 999999 for the maximum number of users that have concurrent sessions on a session host.
        /// Should only be set if the `type` of your Virtual Desktop Host Pool is `Pooled`.
        #[builder(into, default)]
        pub maximum_sessions_allowed: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// `Automatic` assignment – The service will select an available host and assign it to an user. Possible values are `Automatic` and `Direct`. `Direct` Assignment – Admin selects a specific host to assign to an user. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `personal_desktop_assignment_type` is required if the `type` of your Virtual Desktop Host Pool is `Personal`
        #[builder(into, default)]
        pub personal_desktop_assignment_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Option to specify the preferred Application Group type for the Virtual Desktop Host Pool. Valid options are `None`, `Desktop` or `RailApplications`. Default is `Desktop`.
        #[builder(into, default)]
        pub preferred_app_group_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is allowed for the Virtual Desktop Host Pool. Possible values are `Enabled`, `Disabled`, `EnabledForClientsOnly` and `EnabledForSessionHostsOnly`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `scheduled_agent_updates` block as defined below. This enables control of when Agent Updates will be applied to Session Hosts.
        #[builder(into, default)]
        pub scheduled_agent_updates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdates,
            >,
        >,
        /// Enables or disables the Start VM on Connection Feature. Defaults to `false`.
        #[builder(into, default)]
        pub start_vm_on_connect: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Desktop Host Pool. Valid options are `Personal` or `Pooled`. Changing the type forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Allows you to test service changes before they are deployed to production. Defaults to `false`.
        #[builder(into, default)]
        pub validate_environment: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A VM template for session hosts configuration within hostpool. This is a JSON string.
        #[builder(into, default)]
        pub vm_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostPoolResult {
        /// A valid custom RDP properties string for the Virtual Desktop Host Pool, available properties can be [found in this article](https://docs.microsoft.com/windows-server/remote/remote-desktop-services/clients/rdp-files).
        pub custom_rdp_properties: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description for the Virtual Desktop Host Pool.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A friendly name for the Virtual Desktop Host Pool.
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// `BreadthFirst` load balancing distributes new user sessions across all available session hosts in the host pool. Possible values are `BreadthFirst`, `DepthFirst` and `Persistent`.
        /// `DepthFirst` load balancing distributes new user sessions to an available session host with the highest number of connections but has not reached its maximum session limit threshold.
        /// `Persistent` should be used if the host pool type is `Personal`
        pub load_balancer_type: pulumi_gestalt_rust::Output<String>,
        /// The location/region where the Virtual Desktop Host Pool is located. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A valid integer value from 0 to 999999 for the maximum number of users that have concurrent sessions on a session host.
        /// Should only be set if the `type` of your Virtual Desktop Host Pool is `Pooled`.
        pub maximum_sessions_allowed: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// `Automatic` assignment – The service will select an available host and assign it to an user. Possible values are `Automatic` and `Direct`. `Direct` Assignment – Admin selects a specific host to assign to an user. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `personal_desktop_assignment_type` is required if the `type` of your Virtual Desktop Host Pool is `Personal`
        pub personal_desktop_assignment_type: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Option to specify the preferred Application Group type for the Virtual Desktop Host Pool. Valid options are `None`, `Desktop` or `RailApplications`. Default is `Desktop`.
        pub preferred_app_group_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether public network access is allowed for the Virtual Desktop Host Pool. Possible values are `Enabled`, `Disabled`, `EnabledForClientsOnly` and `EnabledForSessionHostsOnly`. Defaults to `Enabled`.
        pub public_network_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `scheduled_agent_updates` block as defined below. This enables control of when Agent Updates will be applied to Session Hosts.
        pub scheduled_agent_updates: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdates,
            >,
        >,
        /// Enables or disables the Start VM on Connection Feature. Defaults to `false`.
        pub start_vm_on_connect: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Desktop Host Pool. Valid options are `Personal` or `Pooled`. Changing the type forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Allows you to test service changes before they are deployed to production. Defaults to `false`.
        pub validate_environment: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A VM template for session hosts configuration within hostpool. This is a JSON string.
        pub vm_template: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostPoolArgs,
    ) -> HostPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_rdp_properties_binding = args
            .custom_rdp_properties
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let friendly_name_binding = args.friendly_name.get_output(context);
        let load_balancer_type_binding = args.load_balancer_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let maximum_sessions_allowed_binding = args
            .maximum_sessions_allowed
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let personal_desktop_assignment_type_binding = args
            .personal_desktop_assignment_type
            .get_output(context);
        let preferred_app_group_type_binding = args
            .preferred_app_group_type
            .get_output(context);
        let public_network_access_binding = args
            .public_network_access
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scheduled_agent_updates_binding = args
            .scheduled_agent_updates
            .get_output(context);
        let start_vm_on_connect_binding = args.start_vm_on_connect.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let validate_environment_binding = args.validate_environment.get_output(context);
        let vm_template_binding = args.vm_template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/hostPool:HostPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customRdpProperties".into(),
                    value: &custom_rdp_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancerType".into(),
                    value: &load_balancer_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumSessionsAllowed".into(),
                    value: &maximum_sessions_allowed_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "personalDesktopAssignmentType".into(),
                    value: &personal_desktop_assignment_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredAppGroupType".into(),
                    value: &preferred_app_group_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduledAgentUpdates".into(),
                    value: &scheduled_agent_updates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startVmOnConnect".into(),
                    value: &start_vm_on_connect_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateEnvironment".into(),
                    value: &validate_environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmTemplate".into(),
                    value: &vm_template_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostPoolResult {
            custom_rdp_properties: o.get_field("customRdpProperties"),
            description: o.get_field("description"),
            friendly_name: o.get_field("friendlyName"),
            load_balancer_type: o.get_field("loadBalancerType"),
            location: o.get_field("location"),
            maximum_sessions_allowed: o.get_field("maximumSessionsAllowed"),
            name: o.get_field("name"),
            personal_desktop_assignment_type: o
                .get_field("personalDesktopAssignmentType"),
            preferred_app_group_type: o.get_field("preferredAppGroupType"),
            public_network_access: o.get_field("publicNetworkAccess"),
            resource_group_name: o.get_field("resourceGroupName"),
            scheduled_agent_updates: o.get_field("scheduledAgentUpdates"),
            start_vm_on_connect: o.get_field("startVmOnConnect"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            validate_environment: o.get_field("validateEnvironment"),
            vm_template: o.get_field("vmTemplate"),
        }
    }
}
