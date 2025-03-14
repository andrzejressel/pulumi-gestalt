/// Provides a resource to manage an [AWS Macie Classification Job](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = account::create("test", AccountArgs::builder().build_struct());
///     let testClassificationJob = classification_job::create(
///         "testClassificationJob",
///         ClassificationJobArgs::builder()
///             .job_type("ONE_TIME")
///             .name("NAME OF THE CLASSIFICATION JOB")
///             .s_3_job_definition(
///                 ClassificationJobS3JobDefinition::builder()
///                     .bucketDefinitions(
///                         vec![
///                             ClassificationJobS3JobDefinitionBucketDefinition::builder()
///                             .accountId("ACCOUNT ID").buckets(vec!["S3 BUCKET NAME",])
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_classification_job` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/classificationJob:ClassificationJob example abcd1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod classification_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClassificationJobArgs {
        /// The custom data identifiers to use for data analysis and classification.
        #[builder(into, default)]
        pub custom_data_identifier_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A custom description of the job. The description can contain as many as 200 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to analyze all existing, eligible objects immediately after the job is created.
        #[builder(into, default)]
        pub initial_run: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The status for the job. Valid values are: `CANCELLED`, `RUNNING` and `USER_PAUSED`
        #[builder(into, default)]
        pub job_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The schedule for running the job. Valid values are: `ONE_TIME` - Run the job only once. If you specify this value, don't specify a value for the `schedule_frequency` property. `SCHEDULED` - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the `schedule_frequency` property to define the recurrence pattern for the job.
        #[builder(into)]
        pub job_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A custom name for the job. The name can contain as many as 500 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The S3 buckets that contain the objects to analyze, and the scope of that analysis. (documented below)
        #[builder(into)]
        pub s3_job_definition: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::macie2::ClassificationJobS3JobDefinition,
        >,
        /// The sampling depth, as a percentage, to apply when processing objects. This value determines the percentage of eligible objects that the job analyzes. If this value is less than 100, Amazon Macie selects the objects to analyze at random, up to the specified percentage, and analyzes all the data in those objects.
        #[builder(into, default)]
        pub sampling_percentage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The recurrence pattern for running the job. To run the job only once, don't specify a value for this property and set the value for the `job_type` property to `ONE_TIME`. (documented below)
        #[builder(into, default)]
        pub schedule_frequency: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::macie2::ClassificationJobScheduleFrequency>,
        >,
        /// A map of key-value pairs that specifies the tags to associate with the job. A job can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClassificationJobResult {
        /// The date and time, in UTC and extended RFC 3339 format, when the job was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The custom data identifiers to use for data analysis and classification.
        pub custom_data_identifier_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A custom description of the job. The description can contain as many as 200 characters.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to analyze all existing, eligible objects immediately after the job is created.
        pub initial_run: pulumi_gestalt_rust::Output<Option<bool>>,
        pub job_arn: pulumi_gestalt_rust::Output<String>,
        pub job_id: pulumi_gestalt_rust::Output<String>,
        /// The status for the job. Valid values are: `CANCELLED`, `RUNNING` and `USER_PAUSED`
        pub job_status: pulumi_gestalt_rust::Output<String>,
        /// The schedule for running the job. Valid values are: `ONE_TIME` - Run the job only once. If you specify this value, don't specify a value for the `schedule_frequency` property. `SCHEDULED` - Run the job on a daily, weekly, or monthly basis. If you specify this value, use the `schedule_frequency` property to define the recurrence pattern for the job.
        pub job_type: pulumi_gestalt_rust::Output<String>,
        /// A custom name for the job. The name can contain as many as 500 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The S3 buckets that contain the objects to analyze, and the scope of that analysis. (documented below)
        pub s3_job_definition: pulumi_gestalt_rust::Output<
            super::super::types::macie2::ClassificationJobS3JobDefinition,
        >,
        /// The sampling depth, as a percentage, to apply when processing objects. This value determines the percentage of eligible objects that the job analyzes. If this value is less than 100, Amazon Macie selects the objects to analyze at random, up to the specified percentage, and analyzes all the data in those objects.
        pub sampling_percentage: pulumi_gestalt_rust::Output<i32>,
        /// The recurrence pattern for running the job. To run the job only once, don't specify a value for this property and set the value for the `job_type` property to `ONE_TIME`. (documented below)
        pub schedule_frequency: pulumi_gestalt_rust::Output<
            super::super::types::macie2::ClassificationJobScheduleFrequency,
        >,
        /// A map of key-value pairs that specifies the tags to associate with the job. A job can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If the current status of the job is `USER_PAUSED`, specifies when the job was paused and when the job or job run will expire and be canceled if it isn't resumed. This value is present only if the value for `job-status` is `USER_PAUSED`.
        pub user_paused_details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::macie2::ClassificationJobUserPausedDetail>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClassificationJobArgs,
    ) -> ClassificationJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_data_identifier_ids_binding = args
            .custom_data_identifier_ids
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let initial_run_binding = args.initial_run.get_output(context);
        let job_status_binding = args.job_status.get_output(context);
        let job_type_binding = args.job_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let s3_job_definition_binding = args.s3_job_definition.get_output(context);
        let sampling_percentage_binding = args.sampling_percentage.get_output(context);
        let schedule_frequency_binding = args.schedule_frequency.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie2/classificationJob:ClassificationJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDataIdentifierIds".into(),
                    value: &custom_data_identifier_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialRun".into(),
                    value: &initial_run_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobStatus".into(),
                    value: &job_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobType".into(),
                    value: &job_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3JobDefinition".into(),
                    value: &s3_job_definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingPercentage".into(),
                    value: &sampling_percentage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleFrequency".into(),
                    value: &schedule_frequency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClassificationJobResult {
            created_at: o.get_field("createdAt"),
            custom_data_identifier_ids: o.get_field("customDataIdentifierIds"),
            description: o.get_field("description"),
            initial_run: o.get_field("initialRun"),
            job_arn: o.get_field("jobArn"),
            job_id: o.get_field("jobId"),
            job_status: o.get_field("jobStatus"),
            job_type: o.get_field("jobType"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            s3_job_definition: o.get_field("s3JobDefinition"),
            sampling_percentage: o.get_field("samplingPercentage"),
            schedule_frequency: o.get_field("scheduleFrequency"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_paused_details: o.get_field("userPausedDetails"),
        }
    }
}
