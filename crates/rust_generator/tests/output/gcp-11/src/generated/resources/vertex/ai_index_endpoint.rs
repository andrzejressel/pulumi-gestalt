/// An endpoint indexes are deployed into. An index endpoint can have multiple deployed indexes.
///
///
/// To get more information about IndexEndpoint, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints/)
///
/// ## Example Usage
///
/// ### Vertex Ai Index Endpoint
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
///     options:
///       dependsOn:
///         - ${vertexVpcConnection}
///   vertexVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vertex_vpc_connection
///     properties:
///       network: ${vertexNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${vertexRange.name}
///   vertexRange:
///     type: gcp:compute:GlobalAddress
///     name: vertex_range
///     properties:
///       name: address-name
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 24
///       network: ${vertexNetwork.id}
///   vertexNetwork:
///     type: gcp:compute:Network
///     name: vertex_network
///     properties:
///       name: network-name
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Index Endpoint With Psc
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       privateServiceConnectConfig:
///         enablePrivateServiceConnect: true
///         projectAllowlists:
///           - ${project.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Index Endpoint With Public Endpoint
///
///
/// ```yaml
/// resources:
///   indexEndpoint:
///     type: gcp:vertex:AiIndexEndpoint
///     name: index_endpoint
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint with an public endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       publicEndpointEnabled: true
/// ```
///
/// ## Import
///
/// IndexEndpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/indexEndpoints/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, IndexEndpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default projects/{{project}}/locations/{{region}}/indexEndpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpoint:AiIndexEndpoint default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_index_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiIndexEndpointArgs {
        /// The description of the Index.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.
        /// Private services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.
        /// [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`.
        /// Where `{project}` is a project number, as in `12345`, and `{network}` is network name.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_service_connect_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiIndexEndpointPrivateServiceConnectConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, the deployed index will be accessible through public endpoint.
        #[builder(into, default)]
        pub public_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The region of the index endpoint. eg us-central1
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiIndexEndpointResult {
        /// The timestamp of when the Index was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The description of the Index.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Used to perform consistent read-modify-write updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The labels with user-defined metadata to organize your Indexes.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the Index.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.
        /// Private services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.
        /// [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`.
        /// Where `{project}` is a project number, as in `12345`, and `{network}` is network name.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        pub private_service_connect_config: pulumi_gestalt_rust::Output<
            super::super::types::vertex::AiIndexEndpointPrivateServiceConnectConfig,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// If publicEndpointEnabled is true, this field will be populated with the domain name to use for this index endpoint.
        pub public_endpoint_domain_name: pulumi_gestalt_rust::Output<String>,
        /// If true, the deployed index will be accessible through public endpoint.
        pub public_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the index endpoint. eg us-central1
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp of when the Index was last updated in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiIndexEndpointArgs,
    ) -> AiIndexEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let network_binding = args.network.get_output(context);
        let private_service_connect_config_binding = args
            .private_service_connect_config
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let public_endpoint_enabled_binding = args
            .public_endpoint_enabled
            .get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiIndexEndpoint:AiIndexEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
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
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateServiceConnectConfig".into(),
                    value: &private_service_connect_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicEndpointEnabled".into(),
                    value: &public_endpoint_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiIndexEndpointResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            private_service_connect_config: o.get_field("privateServiceConnectConfig"),
            project: o.get_field("project"),
            public_endpoint_domain_name: o.get_field("publicEndpointDomainName"),
            public_endpoint_enabled: o.get_field("publicEndpointEnabled"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            update_time: o.get_field("updateTime"),
        }
    }
}
