/// ## Example Usage
///
/// ### Backup Dr Management Server
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: vpc-network
///   privateIpAddress:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_address
///     properties:
///       name: vpc-network
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 20
///       network: ${default.id}
///   defaultConnection:
///     type: gcp:servicenetworking:Connection
///     name: default
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAddress.name}
///   ms-console:
///     type: gcp:backupdisasterrecovery:ManagementServer
///     properties:
///       location: us-central1
///       name: ms-console
///       type: BACKUP_RESTORE
///     options:
///       dependsOn:
///         - ${defaultConnection}
/// ```
///
/// ## Import
///
/// ManagementServer can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/managementServers/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ManagementServer can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default projects/{{project}}/locations/{{location}}/managementServers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:backupdisasterrecovery/managementServer:ManagementServer default {{location}}/{{name}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod management_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementServerArgs {
        /// The location for the management server (management console)
        #[builder(into)]
        pub location: pulumi_gestalt_rust::Input<String>,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        #[builder(into, default)]
        pub networks: pulumi_gestalt_rust::Input<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagementServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The location for the management server (management console)
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The management console URI
        /// Structure is documented below.
        pub management_uris: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::backupdisasterrecovery::ManagementServerManagementUri,
            >,
        >,
        /// The name of management server (management console)
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network details to create management server (management console).
        /// Structure is documented below.
        pub networks: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::backupdisasterrecovery::ManagementServerNetwork>,
            >,
        >,
        /// The oauth2ClientId of management console.
        pub oauth2_client_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The type of management server (management console).
        /// Default value is `BACKUP_RESTORE`.
        /// Possible values are: `BACKUP_RESTORE`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagementServerArgs,
    ) -> ManagementServerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagementServerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ManagementServerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagementServerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ManagementServerResult {
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let networks_binding = args.networks.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let type__binding = args.type_.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:backupdisasterrecovery/managementServer:ManagementServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networks".into(),
                    value: &networks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ManagementServerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            location: o.get_field("location"),
            management_uris: o.get_field("managementUris"),
            name: o.get_field("name"),
            networks: o.get_field("networks"),
            oauth2_client_id: o.get_field("oauth2ClientId"),
            project: o.get_field("project"),
            type_: o.get_field("type"),
        }
    }
}
