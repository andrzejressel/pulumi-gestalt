/// Models are deployed into it, and afterwards Endpoint is called to obtain predictions and explanations.
///
///
/// To get more information about Endpoint, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Endpoint Network
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
///       encryptionSpec:
///         kmsKeyName: kms-name
///       predictRequestResponseLoggingConfig:
///         bigqueryDestination:
///           outputUri: bq://${project.projectId}.${bqDataset.datasetId}.request_response_logging
///         enabled: true
///         samplingRate: 0.1
///       trafficSplit:
///         fn::toJSON:
///           '12345': 100
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
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: kms-name
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-aiplatform.iam.gserviceaccount.com
///   bqDataset:
///     type: gcp:bigquery:Dataset
///     name: bq_dataset
///     properties:
///       datasetId: some_dataset
///       friendlyName: logging dataset
///       description: This is a dataset that requests are logged to
///       location: US
///       deleteContentsOnDestroy: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Endpoint Private Service Connect
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name_8270
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       privateServiceConnectConfig:
///         enablePrivateServiceConnect: true
///         projectAllowlists:
///           - ${project.projectId}
///         enableSecurePrivateServiceConnect: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Endpoint Dedicated Endpoint
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name_41150
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       dedicatedEndpointEnabled: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Endpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/endpoints/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Endpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default projects/{{project}}/locations/{{location}}/endpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiEndpointArgs {
        /// If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon.
        #[builder(into, default)]
        pub dedicated_endpoint_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The description of the Endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiEndpointEncryptionSpec>,
        >,
        /// The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. Only one of the fields, `network` or `privateServiceConnectConfig`, can be set.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configures the request-response logging for online prediction.
        /// Structure is documented below.
        #[builder(into, default)]
        pub predict_request_response_logging_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiEndpointPredictRequestResponseLoggingConfig,
            >,
        >,
        /// Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_service_connect_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vertex::AiEndpointPrivateServiceConnectConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region for the resource
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.
        /// If a DeployedModel's id is not listed in this map, then it receives no traffic.
        /// The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See
        /// the `deployModel` [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and
        /// [documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.
        /// > **Note:** To set the map to empty, set `"{}"`, apply, and then remove the field from your config.
        #[builder(into, default)]
        pub traffic_split: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiEndpointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Output only. Timestamp when the DeployedModel was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. DNS of the dedicated endpoint. Will only be populated if dedicatedEndpointEnabled is true. Format: `https://{endpointId}.{region}-{projectNumber}.prediction.vertexai.goog`.
        pub dedicated_endpoint_dns: pulumi_gestalt_rust::Output<String>,
        /// If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon.
        pub dedicated_endpoint_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/).
        /// Structure is documented below.
        pub deployed_models: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vertex::AiEndpointDeployedModel>,
        >,
        /// The description of the Endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key.
        /// Structure is documented below.
        pub encryption_spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiEndpointEncryptionSpec>,
        >,
        /// Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}`
        pub model_deployment_monitoring_job: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. Only one of the fields, `network` or `privateServiceConnectConfig`, can be set.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configures the request-response logging for online prediction.
        /// Structure is documented below.
        pub predict_request_response_logging_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::vertex::AiEndpointPredictRequestResponseLoggingConfig,
            >,
        >,
        /// Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        pub private_service_connect_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::vertex::AiEndpointPrivateServiceConnectConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region for the resource
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.
        /// If a DeployedModel's id is not listed in this map, then it receives no traffic.
        /// The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See
        /// the `deployModel` [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and
        /// [documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.
        /// > **Note:** To set the map to empty, set `"{}"`, apply, and then remove the field from your config.
        pub traffic_split: pulumi_gestalt_rust::Output<String>,
        /// Output only. Timestamp when this Endpoint was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AiEndpointArgs,
    ) -> AiEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dedicated_endpoint_enabled_binding = args
            .dedicated_endpoint_enabled
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let encryption_spec_binding = args.encryption_spec.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let predict_request_response_logging_config_binding = args
            .predict_request_response_logging_config
            .get_output(context);
        let private_service_connect_config_binding = args
            .private_service_connect_config
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let traffic_split_binding = args.traffic_split.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:vertex/aiEndpoint:AiEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dedicatedEndpointEnabled".into(),
                    value: &dedicated_endpoint_enabled_binding.drop_type(),
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
                    name: "encryptionSpec".into(),
                    value: &encryption_spec_binding.drop_type(),
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "predictRequestResponseLoggingConfig".into(),
                    value: &predict_request_response_logging_config_binding.drop_type(),
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
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficSplit".into(),
                    value: &traffic_split_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AiEndpointResult {
            id: o.get_field("id"),
            create_time: o.get_field("createTime"),
            dedicated_endpoint_dns: o.get_field("dedicatedEndpointDns"),
            dedicated_endpoint_enabled: o.get_field("dedicatedEndpointEnabled"),
            deployed_models: o.get_field("deployedModels"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            encryption_spec: o.get_field("encryptionSpec"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            model_deployment_monitoring_job: o.get_field("modelDeploymentMonitoringJob"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            predict_request_response_logging_config: o
                .get_field("predictRequestResponseLoggingConfig"),
            private_service_connect_config: o.get_field("privateServiceConnectConfig"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            traffic_split: o.get_field("trafficSplit"),
            update_time: o.get_field("updateTime"),
        }
    }
}
