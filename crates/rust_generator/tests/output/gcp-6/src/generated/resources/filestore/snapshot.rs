/// A Google Cloud Filestore snapshot.
///
///
/// To get more information about Snapshot, see:
///
/// * [API documentation](https://cloud.google.com/filestore/docs/reference/rest/v1/projects.locations.instances.snapshots)
/// * How-to Guides
///     * [Creating Snapshots](https://cloud.google.com/filestore/docs/create-snapshots)
///     * [Official Documentation](https://cloud.google.com/filestore/docs/snapshots)
///
/// ## Example Usage
///
/// ### Filestore Snapshot Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(1024)
///                     .name("share1")
///                     .build_struct(),
///             )
///             .location("us-east1")
///             .name("test-instance-for-snapshot")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().modes(vec!["MODE_IPV4",])
///                     .network("default").build_struct(),
///                 ],
///             )
///             .tier("ENTERPRISE")
///             .build_struct(),
///     );
///     let snapshot = snapshot::create(
///         "snapshot",
///         SnapshotArgs::builder()
///             .instance("${instance.name}")
///             .location("us-east1")
///             .name("test-snapshot")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Filestore Snapshot Full
///
///
/// ```yaml
/// resources:
///   snapshot:
///     type: gcp:filestore:Snapshot
///     properties:
///       name: test-snapshot
///       instance: ${instance.name}
///       location: us-west1
///       description: Snapshot of test-instance-for-snapshot
///       labels:
///         my_label: value
///   instance:
///     type: gcp:filestore:Instance
///     properties:
///       name: test-instance-for-snapshot
///       location: us-west1
///       tier: ENTERPRISE
///       fileShares:
///         capacityGb: 1024
///         name: share1
///       networks:
///         - network: default
///           modes:
///             - MODE_IPV4
/// ```
///
/// ## Import
///
/// Snapshot can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{instance}}/snapshots/{{name}}`
///
/// * `{{project}}/{{location}}/{{instance}}/{{name}}`
///
/// * `{{location}}/{{instance}}/{{name}}`
///
/// When using the `pulumi import` command, Snapshot can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:filestore/snapshot:Snapshot default projects/{{project}}/locations/{{location}}/instances/{{instance}}/snapshots/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/snapshot:Snapshot default {{project}}/{{location}}/{{instance}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/snapshot:Snapshot default {{location}}/{{instance}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the filestore instance.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the snapshot. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The time when the snapshot was created in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the snapshot with 2048 characters or less. Requests with longer descriptions will be rejected.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The amount of bytes needed to allocate a full copy of the snapshot content.
        pub filesystem_used_bytes: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the filestore instance.
        ///
        ///
        /// - - -
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the snapshot. The name must be unique within the specified instance.
        /// The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The snapshot state.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:filestore/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: &instance_binding.drop_type(),
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
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            id: o.get_field("id"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            filesystem_used_bytes: o.get_field("filesystemUsedBytes"),
            instance: o.get_field("instance"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
        }
    }
}
