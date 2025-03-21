/// An Integration connectors Managed Zone.
///
///
/// To get more information about ManagedZone, see:
///
/// * [API documentation](https://cloud.google.com/integration-connectors/docs/reference/rest/v1/projects.locations.global.managedZones)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/integration-connectors/docs)
///
/// ## Example Usage
///
/// ### Integration Connectors Managed Zone
///
///
/// ```yaml
/// resources:
///   targetProject:
///     type: gcp:organizations:Project
///     name: target_project
///     properties:
///       projectId: tf-test_40785
///       name: tf-test_79169
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   dnsPeerBinding:
///     type: gcp:projects:IAMMember
///     name: dns_peer_binding
///     properties:
///       project: ${targetProject.projectId}
///       role: roles/dns.peer
///       member: serviceAccount:service-${testProject.number}@gcp-sa-connectors.iam.gserviceaccount.com
///   dns:
///     type: gcp:projects:Service
///     properties:
///       project: ${targetProject.projectId}
///       service: dns.googleapis.com
///   compute:
///     type: gcp:projects:Service
///     properties:
///       project: ${targetProject.projectId}
///       service: compute.googleapis.com
///   network:
///     type: gcp:compute:Network
///     properties:
///       project: ${targetProject.projectId}
///       name: test
///       autoCreateSubnetworks: false
///     options:
///       dependsOn:
///         - ${compute}
///   zone:
///     type: gcp:dns:ManagedZone
///     properties:
///       name: tf-test-dns_56529
///       dnsName: private_75413.example.com.
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${network.id}
///     options:
///       dependsOn:
///         - ${dns}
///   testmanagedzone:
///     type: gcp:integrationconnectors:ManagedZone
///     properties:
///       name: test
///       description: tf created description
///       labels:
///         intent: example
///       targetProject: ${targetProject.projectId}
///       targetVpc: test
///       dns: ${zone.dnsName}
///     options:
///       dependsOn:
///         - ${dnsPeerBinding}
///         - ${zone}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// ManagedZone can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/managedZones/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ManagedZone can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default projects/{{project}}/locations/global/managedZones/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:integrationconnectors/managedZone:ManagedZone default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedZoneArgs {
        /// Description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DNS Name of the resource.
        #[builder(into)]
        pub dns: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of Managed Zone needs to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Target Project.
        #[builder(into)]
        pub target_project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Target Project VPC Network.
        #[builder(into)]
        pub target_vpc: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedZoneResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// DNS Name of the resource.
        pub dns: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of Managed Zone needs to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Target Project.
        pub target_project: pulumi_gestalt_rust::Output<String>,
        /// The name of the Target Project VPC Network.
        pub target_vpc: pulumi_gestalt_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedZoneArgs,
    ) -> ManagedZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let dns_binding = args.dns.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let target_project_binding = args.target_project.get_output(context);
        let target_vpc_binding = args.target_vpc.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:integrationconnectors/managedZone:ManagedZone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dns".into(),
                    value: &dns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetProject".into(),
                    value: &target_project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVpc".into(),
                    value: &target_vpc_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedZoneResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            dns: o.get_field("dns"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            target_project: o.get_field("targetProject"),
            target_vpc: o.get_field("targetVpc"),
            update_time: o.get_field("updateTime"),
        }
    }
}
