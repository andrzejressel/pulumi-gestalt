/// Resource for managing an AWS RDS (Relational Database) Export Task.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = export_task::create(
///         "example",
///         ExportTaskArgs::builder()
///             .export_task_identifier("example")
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .kms_key_id("${exampleAwsKmsKey.arn}")
///             .s_3_bucket_name("${exampleAwsS3Bucket.id}")
///             .source_arn("${exampleAwsDbSnapshot.dbSnapshotArn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Complete Usage
///
/// ```yaml
/// resources:
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example
///       forceDestroy: true
///   exampleBucketAclV2:
///     type: aws:s3:BucketAclV2
///     name: example
///     properties:
///       bucket: ${exampleBucketV2.id}
///       acl: private
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: export.rds.amazonaws.com
///   examplePolicy:
///     type: aws:iam:Policy
///     name: example
///     properties:
///       name: example
///       policy: ${example.json}
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policyArn: ${examplePolicy.arn}
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       deletionWindowInDays: 10
///   exampleInstance:
///     type: aws:rds:Instance
///     name: example
///     properties:
///       identifier: example
///       allocatedStorage: 10
///       dbName: test
///       engine: mysql
///       engineVersion: '5.7'
///       instanceClass: db.t3.micro
///       username: foo
///       password: foobarbaz
///       parameterGroupName: default.mysql5.7
///       skipFinalSnapshot: true
///   exampleSnapshot:
///     type: aws:rds:Snapshot
///     name: example
///     properties:
///       dbInstanceIdentifier: ${exampleInstance.identifier}
///       dbSnapshotIdentifier: example
///   exampleExportTask:
///     type: aws:rds:ExportTask
///     name: example
///     properties:
///       exportTaskIdentifier: example
///       sourceArn: ${exampleSnapshot.dbSnapshotArn}
///       s3BucketName: ${exampleBucketV2.id}
///       iamRoleArn: ${exampleRole.arn}
///       kmsKeyId: ${exampleKey.arn}
///       exportOnlies:
///         - database
///       s3Prefix: my_prefix/example
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - s3:ListAllMyBuckets
///             resources:
///               - '*'
///           - actions:
///               - s3:GetBucketLocation
///               - s3:ListBucket
///             resources:
///               - ${exampleBucketV2.arn}
///           - actions:
///               - s3:GetObject
///               - s3:PutObject
///               - s3:DeleteObject
///             resources:
///               - ${exampleBucketV2.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a RDS (Relational Database) Export Task using the `export_task_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/exportTask:ExportTask example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod export_task {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExportTaskArgs {
        /// Data to be exported from the snapshot. If this parameter is not provided, all the snapshot data is exported. Valid values are documented in the [AWS StartExportTask API documentation](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_StartExportTask.html#API_StartExportTask_RequestParameters).
        #[builder(into, default)]
        pub export_onlies: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Unique identifier for the snapshot export task.
        #[builder(into)]
        pub export_task_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the IAM role to use for writing to the Amazon S3 bucket.
        #[builder(into)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Amazon Web Services KMS key to use to encrypt the snapshot.
        #[builder(into)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Amazon S3 bucket to export the snapshot to.
        #[builder(into)]
        pub s3_bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon S3 bucket prefix to use as the file name and path of the exported snapshot.
        #[builder(into, default)]
        pub s3_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the snapshot to export.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub source_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ExportTaskTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExportTaskResult {
        /// Data to be exported from the snapshot. If this parameter is not provided, all the snapshot data is exported. Valid values are documented in the [AWS StartExportTask API documentation](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_StartExportTask.html#API_StartExportTask_RequestParameters).
        pub export_onlies: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Unique identifier for the snapshot export task.
        pub export_task_identifier: pulumi_gestalt_rust::Output<String>,
        /// Reason the export failed, if it failed.
        pub failure_cause: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role to use for writing to the Amazon S3 bucket.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Amazon Web Services KMS key to use to encrypt the snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Progress of the snapshot export task as a percentage.
        pub percent_progress: pulumi_gestalt_rust::Output<i32>,
        /// Name of the Amazon S3 bucket to export the snapshot to.
        pub s3_bucket_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon S3 bucket prefix to use as the file name and path of the exported snapshot.
        pub s3_prefix: pulumi_gestalt_rust::Output<String>,
        /// Time that the snapshot was created.
        pub snapshot_time: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the snapshot to export.
        ///
        /// The following arguments are optional:
        pub source_arn: pulumi_gestalt_rust::Output<String>,
        /// Type of source for the export.
        pub source_type: pulumi_gestalt_rust::Output<String>,
        /// Status of the export task.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Time that the snapshot export task completed.
        pub task_end_time: pulumi_gestalt_rust::Output<String>,
        /// Time that the snapshot export task started.
        pub task_start_time: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::ExportTaskTimeouts>,
        >,
        /// Warning about the snapshot export task, if any.
        pub warning_message: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExportTaskArgs,
    ) -> ExportTaskResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let export_onlies_binding = args.export_onlies.get_output(context);
        let export_task_identifier_binding = args
            .export_task_identifier
            .get_output(context);
        let iam_role_arn_binding = args.iam_role_arn.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let s3_bucket_name_binding = args.s3_bucket_name.get_output(context);
        let s3_prefix_binding = args.s3_prefix.get_output(context);
        let source_arn_binding = args.source_arn.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/exportTask:ExportTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportOnlies".into(),
                    value: &export_onlies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportTaskIdentifier".into(),
                    value: &export_task_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3BucketName".into(),
                    value: &s3_bucket_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3Prefix".into(),
                    value: &s3_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceArn".into(),
                    value: &source_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExportTaskResult {
            export_onlies: o.get_field("exportOnlies"),
            export_task_identifier: o.get_field("exportTaskIdentifier"),
            failure_cause: o.get_field("failureCause"),
            iam_role_arn: o.get_field("iamRoleArn"),
            kms_key_id: o.get_field("kmsKeyId"),
            percent_progress: o.get_field("percentProgress"),
            s3_bucket_name: o.get_field("s3BucketName"),
            s3_prefix: o.get_field("s3Prefix"),
            snapshot_time: o.get_field("snapshotTime"),
            source_arn: o.get_field("sourceArn"),
            source_type: o.get_field("sourceType"),
            status: o.get_field("status"),
            task_end_time: o.get_field("taskEndTime"),
            task_start_time: o.get_field("taskStartTime"),
            timeouts: o.get_field("timeouts"),
            warning_message: o.get_field("warningMessage"),
        }
    }
}
