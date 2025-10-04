/// An `Environment` in Apigee.
///
///
/// To get more information about Environment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ### Apigee Environment Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   env:
///     type: gcp:apigee:Environment
///     properties:
///       name: my-environment
///       description: Apigee Environment
///       displayName: environment-1
///       orgId: ${apigeeOrg.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `{{org_id}}/environments/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/environment:Environment default {{org_id}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/environment:Environment default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Optional. API Proxy type supported by the environment. The type can be set when creating
        /// the Environment and cannot be changed.
        /// Possible values are: `API_PROXY_TYPE_UNSPECIFIED`, `PROGRAMMABLE`, `CONFIGURABLE`.
        #[builder(into, default)]
        pub api_proxy_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Deployment type supported by the environment. The deployment type can be
        /// set when creating the environment and cannot be changed. When you enable archive
        /// deployment, you will be prevented from performing a subset of actions within the
        /// environment, including:
        /// Managing the deployment of API proxy or shared flow revisions;
        /// Creating, updating, or deleting resource files;
        /// Creating, updating, or deleting target servers.
        /// Possible values are: `DEPLOYMENT_TYPE_UNSPECIFIED`, `PROXY`, `ARCHIVE`.
        #[builder(into, default)]
        pub deployment_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the environment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name of the environment.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of "http" or "https", and the port must be supplied.
        #[builder(into, default)]
        pub forward_proxy_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the environment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NodeConfig for setting the min/max number of nodes associated with the environment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub node_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigee::EnvironmentNodeConfig>,
        >,
        /// The Apigee Organization associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Types that can be selected for an Environment. Each of the types are
        /// limited by capability and capacity. Refer to Apigee's public documentation
        /// to understand about each of these types in details.
        /// An Apigee org can support heterogeneous Environments.
        /// Possible values are: `ENVIRONMENT_TYPE_UNSPECIFIED`, `BASE`, `INTERMEDIATE`, `COMPREHENSIVE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Optional. API Proxy type supported by the environment. The type can be set when creating
        /// the Environment and cannot be changed.
        /// Possible values are: `API_PROXY_TYPE_UNSPECIFIED`, `PROGRAMMABLE`, `CONFIGURABLE`.
        pub api_proxy_type: pulumi_gestalt_rust::Output<String>,
        /// Optional. Deployment type supported by the environment. The deployment type can be
        /// set when creating the environment and cannot be changed. When you enable archive
        /// deployment, you will be prevented from performing a subset of actions within the
        /// environment, including:
        /// Managing the deployment of API proxy or shared flow revisions;
        /// Creating, updating, or deleting resource files;
        /// Creating, updating, or deleting target servers.
        /// Possible values are: `DEPLOYMENT_TYPE_UNSPECIFIED`, `PROXY`, `ARCHIVE`.
        pub deployment_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the environment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Display name of the environment.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of "http" or "https", and the port must be supplied.
        pub forward_proxy_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the environment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// NodeConfig for setting the min/max number of nodes associated with the environment.
        /// Structure is documented below.
        pub node_config: pulumi_gestalt_rust::Output<
            super::super::types::apigee::EnvironmentNodeConfig,
        >,
        /// The Apigee Organization associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Types that can be selected for an Environment. Each of the types are
        /// limited by capability and capacity. Refer to Apigee's public documentation
        /// to understand about each of these types in details.
        /// An Apigee org can support heterogeneous Environments.
        /// Possible values are: `ENVIRONMENT_TYPE_UNSPECIFIED`, `BASE`, `INTERMEDIATE`, `COMPREHENSIVE`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentArgs,
    ) -> EnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_proxy_type_binding = args.api_proxy_type.get_output(context);
        let deployment_type_binding = args.deployment_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let forward_proxy_uri_binding = args.forward_proxy_uri.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_config_binding = args.node_config.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/environment:Environment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiProxyType".into(),
                    value: &api_proxy_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentType".into(),
                    value: &deployment_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardProxyUri".into(),
                    value: &forward_proxy_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeConfig".into(),
                    value: &node_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentResult {
            id: o.get_field("id"),
            api_proxy_type: o.get_field("apiProxyType"),
            deployment_type: o.get_field("deploymentType"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            forward_proxy_uri: o.get_field("forwardProxyUri"),
            name: o.get_field("name"),
            node_config: o.get_field("nodeConfig"),
            org_id: o.get_field("orgId"),
            type_: o.get_field("type"),
        }
    }
}
