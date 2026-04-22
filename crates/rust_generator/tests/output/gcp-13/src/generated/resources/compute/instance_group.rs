/// Creates a group of dissimilar Compute Engine virtual machine instances.
/// For more information, see [the official documentation](https://cloud.google.com/compute/docs/instance-groups/#unmanaged_instance_groups)
/// and [API](https://cloud.google.com/compute/docs/reference/latest/instanceGroups)
///
///
/// ## Example Usage
///
/// ### Empty Instance Group
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = instance_group::create(
///         "test",
///         InstanceGroupArgs::builder()
///             .description("Test instance group")
///             .name("test")
///             .network("${default.id}")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Usage - With instances and named ports
///
/// ```yaml
/// resources:
///   webservers:
///     type: gcp:compute:InstanceGroup
///     properties:
///       name: webservers
///       description: Test instance group
///       instances:
///         - ${test.id}
///         - ${test2.id}
///       namedPorts:
///         - name: http
///           port: '8080'
///         - name: https
///           port: '8443'
///       zone: us-central1-a
/// ```
///
/// ## Import
///
/// Instance groups can be imported using the `zone` and `name` with an optional `project`, e.g.
///
/// * `projects/{{project_id}}/zones/{{zone}}/instanceGroups/{{instance_group_id}}`
///
/// * `{{project_id}}/{{zone}}/{{instance_group_id}}`
///
/// * `{{zone}}/{{instance_group_id}}`
///
/// When using the `pulumi import` command, instance groups can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default {{zone}}/{{instance_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default {{project_id}}/{{zone}}/{{instance_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceGroup:InstanceGroup default projects/{{project_id}}/zones/{{zone}}/instanceGroups/{{instance_group_id}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod instance_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceGroupArgs {
        /// An optional textual description of the instance
        /// group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The list of instances in the group, in `self_link` format.
        /// When adding instances they must all be in the same network and zone as the instance group.
        #[builder(into, default)]
        pub instances: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// The name of the instance group. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The named port configuration. See the section below
        /// for details on configuration. Structure is documented below.
        #[builder(into, default)]
        pub named_ports: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::compute::InstanceGroupNamedPort>>,
        >,
        /// The URL of the network the instance group is in. If
        /// this is different from the network where the instances are in, the creation
        /// fails. Defaults to the network where the instances are in (if neither
        /// `network` nor `instances` is specified, this field will be blank).
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The zone that this instance group should be created in.
        ///
        /// - - -
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An optional textual description of the instance
        /// group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of instances in the group, in `self_link` format.
        /// When adding instances they must all be in the same network and zone as the instance group.
        pub instances: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the instance group. Must be 1-63
        /// characters long and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Supported characters
        /// include lowercase letters, numbers, and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The named port configuration. See the section below
        /// for details on configuration. Structure is documented below.
        pub named_ports: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceGroupNamedPort>>,
        >,
        /// The URL of the network the instance group is in. If
        /// this is different from the network where the instances are in, the creation
        /// fails. Defaults to the network where the instances are in (if neither
        /// `network` nor `instances` is specified, this field will be blank).
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The number of instances in the group.
        pub size: pulumi_gestalt_rust::Output<i32>,
        /// The zone that this instance group should be created in.
        ///
        /// - - -
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupArgs,
    ) -> InstanceGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> InstanceGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> InstanceGroupResult {
        let description_binding = args.description.get_output(ctx);
        let instances_binding = args.instances.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let named_ports_binding = args.named_ports.get_output(ctx);
        let network_binding = args.network.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let zone_binding = args.zone.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceGroup:InstanceGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instances".into(),
                    value: &instances_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namedPorts".into(),
                    value: &named_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        InstanceGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            description: o.get_field("description"),
            instances: o.get_field("instances"),
            name: o.get_field("name"),
            named_ports: o.get_field("namedPorts"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            size: o.get_field("size"),
            zone: o.get_field("zone"),
        }
    }
}
