/// ## Example Usage
///
/// ### Network Security Intercept Deployment Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: example-network
///       autoCreateSubnetworks: false
///   subnetwork:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: example-subnet
///       region: us-central1
///       ipCidrRange: 10.1.0.0/16
///       network: ${network.name}
///   healthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: health_check
///     properties:
///       name: example-hc
///       region: us-central1
///       httpHealthCheck:
///         port: 80
///   backendService:
///     type: gcp:compute:RegionBackendService
///     name: backend_service
///     properties:
///       name: example-bs
///       region: us-central1
///       healthChecks: ${healthCheck.id}
///       protocol: UDP
///       loadBalancingScheme: INTERNAL
///   forwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: forwarding_rule
///     properties:
///       name: example-fwr
///       region: us-central1
///       network: ${network.name}
///       subnetwork: ${subnetwork.name}
///       backendService: ${backendService.id}
///       loadBalancingScheme: INTERNAL
///       ports:
///         - 6081
///       ipProtocol: UDP
///   deploymentGroup:
///     type: gcp:networksecurity:InterceptDeploymentGroup
///     name: deployment_group
///     properties:
///       interceptDeploymentGroupId: example-dg
///       location: global
///       network: ${network.id}
///   default:
///     type: gcp:networksecurity:InterceptDeployment
///     properties:
///       interceptDeploymentId: example-deployment
///       location: us-central1-a
///       forwardingRule: ${forwardingRule.id}
///       interceptDeploymentGroup: ${deploymentGroup.id}
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// InterceptDeployment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/interceptDeployments/{{intercept_deployment_id}}`
///
/// * `{{project}}/{{location}}/{{intercept_deployment_id}}`
///
/// * `{{location}}/{{intercept_deployment_id}}`
///
/// When using the `pulumi import` command, InterceptDeployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeployment:InterceptDeployment default projects/{{project}}/locations/{{location}}/interceptDeployments/{{intercept_deployment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeployment:InterceptDeployment default {{project}}/{{location}}/{{intercept_deployment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeployment:InterceptDeployment default {{location}}/{{intercept_deployment_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod intercept_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InterceptDeploymentArgs {
        /// Immutable. The regional load balancer which the intercepted traffic should be forwarded
        /// to. Format is:
        /// projects/{project}/regions/{region}/forwardingRules/{forwardingRule}
        #[builder(into)]
        pub forwarding_rule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Immutable. The Intercept Deployment Group that this resource is part of. Format is:
        /// `projects/{project}/locations/global/interceptDeploymentGroups/{interceptDeploymentGroup}`
        #[builder(into)]
        pub intercept_deployment_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// intercept_deployment_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        #[builder(into)]
        pub intercept_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/InterceptDeployment`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InterceptDeploymentResult {
        /// Create time stamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. The regional load balancer which the intercepted traffic should be forwarded
        /// to. Format is:
        /// projects/{project}/regions/{region}/forwardingRules/{forwardingRule}
        pub forwarding_rule: pulumi_gestalt_rust::Output<String>,
        /// Immutable. The Intercept Deployment Group that this resource is part of. Format is:
        /// `projects/{project}/locations/global/interceptDeploymentGroups/{interceptDeploymentGroup}`
        pub intercept_deployment_group: pulumi_gestalt_rust::Output<String>,
        /// Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// intercept_deployment_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        pub intercept_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/InterceptDeployment`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the InterceptDeployment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether reconciling is in progress, recommended per
        /// https://google.aip.dev/128.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Current state of the deployment.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// ACTIVE
        /// CREATING
        /// DELETING
        /// OUT_OF_SYNC
        /// DELETE_FAILED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Update time stamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InterceptDeploymentArgs,
    ) -> InterceptDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let forwarding_rule_binding = args.forwarding_rule.get_output(context);
        let intercept_deployment_group_binding = args
            .intercept_deployment_group
            .get_output(context);
        let intercept_deployment_id_binding = args
            .intercept_deployment_id
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/interceptDeployment:InterceptDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardingRule".into(),
                    value: &forwarding_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interceptDeploymentGroup".into(),
                    value: &intercept_deployment_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interceptDeploymentId".into(),
                    value: &intercept_deployment_id_binding.drop_type(),
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
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InterceptDeploymentResult {
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            forwarding_rule: o.get_field("forwardingRule"),
            intercept_deployment_group: o.get_field("interceptDeploymentGroup"),
            intercept_deployment_id: o.get_field("interceptDeploymentId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
