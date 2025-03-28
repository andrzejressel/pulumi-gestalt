/// Provides a CloudWatch Evidently Project resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       tags:
///         Key1: example Project
/// ```
///
/// ### Store evaluation events in a CloudWatch Log Group
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       dataDelivery:
///         cloudwatchLogs:
///           logGroup: example-log-group-name
///       tags:
///         Key1: example Project
/// ```
///
/// ### Store evaluation events in an S3 bucket
///
/// ```yaml
/// resources:
///   example:
///     type: aws:evidently:Project
///     properties:
///       name: Example
///       description: Example Description
///       dataDelivery:
///         s3Destination:
///           bucket: example-bucket-name
///           prefix: example
///       tags:
///         Key1: example Project
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Evidently Project using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:evidently/project:Project example arn:aws:evidently:us-east-1:123456789012:segment/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// A block that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view. See below.
        #[builder(into, default)]
        pub data_delivery: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::evidently::ProjectDataDelivery>,
        >,
        /// Specifies the description of the project.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name for the project.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the project. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The number of ongoing experiments currently in the project.
        pub active_experiment_count: pulumi_gestalt_rust::Output<i32>,
        /// The number of ongoing launches currently in the project.
        pub active_launch_count: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the project.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the project is created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// A block that contains information about where Evidently is to store evaluation events for longer term storage, if you choose to do so. If you choose not to store these events, Evidently deletes them after using them to produce metrics and other experiment results that you can view. See below.
        pub data_delivery: pulumi_gestalt_rust::Output<
            Option<super::super::types::evidently::ProjectDataDelivery>,
        >,
        /// Specifies the description of the project.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of experiments currently in the project. This includes all experiments that have been created and not deleted, whether they are ongoing or not.
        pub experiment_count: pulumi_gestalt_rust::Output<i32>,
        /// The number of features currently in the project.
        pub feature_count: pulumi_gestalt_rust::Output<i32>,
        /// The date and time that the project was most recently updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// The number of launches currently in the project. This includes all launches that have been created and not deleted, whether they are ongoing or not.
        pub launch_count: pulumi_gestalt_rust::Output<i32>,
        /// A name for the project.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The current state of the project. Valid values are `AVAILABLE` and `UPDATING`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the project. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_delivery_binding = args.data_delivery.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:evidently/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataDelivery".into(),
                    value: &data_delivery_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            active_experiment_count: o.get_field("activeExperimentCount"),
            active_launch_count: o.get_field("activeLaunchCount"),
            arn: o.get_field("arn"),
            created_time: o.get_field("createdTime"),
            data_delivery: o.get_field("dataDelivery"),
            description: o.get_field("description"),
            experiment_count: o.get_field("experimentCount"),
            feature_count: o.get_field("featureCount"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            launch_count: o.get_field("launchCount"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
