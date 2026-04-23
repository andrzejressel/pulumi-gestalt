/// The PrivateConnection resource is used to establish private connectivity between Datastream and a customer's network.
///
///
/// To get more information about PrivateConnection, see:
///
/// * [API documentation](https://cloud.google.com/datastream/docs/reference/rest/v1/projects.locations.privateConnections)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/datastream/docs/create-a-private-connectivity-configuration)
///
/// ## Example Usage
///
/// ### Datastream Private Connection Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:datastream:PrivateConnection
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       privateConnectionId: my-connection
///       labels:
///         key: value
///       vpcPeeringConfig:
///         vpc: ${defaultNetwork.id}
///         subnet: 10.0.0.0/29
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: my-network
/// ```
///
/// ## Import
///
/// PrivateConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/privateConnections/{{private_connection_id}}`
///
/// * `{{project}}/{{location}}/{{private_connection_id}}`
///
/// * `{{location}}/{{private_connection_id}}`
///
/// When using the `pulumi import` command, PrivateConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default projects/{{project}}/locations/{{location}}/privateConnections/{{private_connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default {{project}}/{{location}}/{{private_connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default {{location}}/{{private_connection_id}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod private_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateConnectionArgs {
        /// If set to true, will skip validations.
        #[builder(into, default)]
        pub create_without_validation: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Display name.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::Input<String>,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this private connection is located in.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::Input<String>,
        /// The private connectivity identifier.
        #[builder(into)]
        pub private_connection_id: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The VPC Peering configuration is used to create VPC peering
        /// between Datastream and the consumer's VPC.
        /// Structure is documented below.
        #[builder(into)]
        pub vpc_peering_config: pulumi_gestalt_rust::Input<
            super::super::types::datastream::PrivateConnectionVpcPeeringConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateConnectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// If set to true, will skip validations.
        pub create_without_validation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The PrivateConnection error in case of failure.
        /// Structure is documented below.
        pub errors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datastream::PrivateConnectionError>,
        >,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this private connection is located in.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The private connectivity identifier.
        pub private_connection_id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// State of the PrivateConnection.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The VPC Peering configuration is used to create VPC peering
        /// between Datastream and the consumer's VPC.
        /// Structure is documented below.
        pub vpc_peering_config: pulumi_gestalt_rust::Output<
            super::super::types::datastream::PrivateConnectionVpcPeeringConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateConnectionArgs,
    ) -> PrivateConnectionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateConnectionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PrivateConnectionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrivateConnectionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PrivateConnectionResult {
        let create_without_validation_binding = args
            .create_without_validation
            .get_output(ctx);
        let display_name_binding = args.display_name.get_output(ctx);
        let labels_binding = args.labels.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let private_connection_id_binding = args.private_connection_id.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let vpc_peering_config_binding = args.vpc_peering_config.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datastream/privateConnection:PrivateConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createWithoutValidation".into(),
                    value: &create_without_validation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateConnectionId".into(),
                    value: &private_connection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcPeeringConfig".into(),
                    value: &vpc_peering_config_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        PrivateConnectionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            create_without_validation: o.get_field("createWithoutValidation"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            errors: o.get_field("errors"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_connection_id: o.get_field("privateConnectionId"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
            vpc_peering_config: o.get_field("vpcPeeringConfig"),
        }
    }
}
