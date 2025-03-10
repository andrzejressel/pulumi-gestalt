/// Resource for managing an AWS Redshift Logging configuration.
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
///     let example = logging::create(
///         "example",
///         LoggingArgs::builder()
///             .cluster_identifier("${exampleAwsRedshiftCluster.id}")
///             .log_destination_type("cloudwatch")
///             .log_exports(vec!["connectionlog", "userlog",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### S3 Destination Type
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = logging::create(
///         "example",
///         LoggingArgs::builder()
///             .bucket_name("${exampleAwsS3Bucket.id}")
///             .cluster_identifier("${exampleAwsRedshiftCluster.id}")
///             .log_destination_type("s3")
///             .s_3_key_prefix("example-prefix/")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Logging using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/logging:Logging example cluster-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logging {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingArgs {
        /// Name of an existing S3 bucket where the log files are to be stored. Required when `log_destination_type` is `s3`. Must be in the same region as the cluster and the cluster must have read bucket and put object permissions. For more information on the permissions required for the bucket, please read the AWS [documentation](http://docs.aws.amazon.com/redshift/latest/mgmt/db-auditing.html#db-auditing-enable-logging)
        #[builder(into, default)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the source cluster.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Log destination type. Valid values are `s3` and `cloudwatch`.
        #[builder(into, default)]
        pub log_destination_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Collection of exported log types. Required when `log_destination_type` is `cloudwatch`. Valid values are `connectionlog`, `useractivitylog`, and `userlog`.
        #[builder(into, default)]
        pub log_exports: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Prefix applied to the log file names.
        #[builder(into, default)]
        pub s3_key_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoggingResult {
        /// Name of an existing S3 bucket where the log files are to be stored. Required when `log_destination_type` is `s3`. Must be in the same region as the cluster and the cluster must have read bucket and put object permissions. For more information on the permissions required for the bucket, please read the AWS [documentation](http://docs.aws.amazon.com/redshift/latest/mgmt/db-auditing.html#db-auditing-enable-logging)
        pub bucket_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the source cluster.
        ///
        /// The following arguments are optional:
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Log destination type. Valid values are `s3` and `cloudwatch`.
        pub log_destination_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Collection of exported log types. Required when `log_destination_type` is `cloudwatch`. Valid values are `connectionlog`, `useractivitylog`, and `userlog`.
        pub log_exports: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Prefix applied to the log file names.
        pub s3_key_prefix: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggingArgs,
    ) -> LoggingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_name_binding = args.bucket_name.get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let log_destination_type_binding = args.log_destination_type.get_output(context);
        let log_exports_binding = args.log_exports.get_output(context);
        let s3_key_prefix_binding = args.s3_key_prefix.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/logging:Logging".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logDestinationType".into(),
                    value: &log_destination_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logExports".into(),
                    value: &log_exports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3KeyPrefix".into(),
                    value: &s3_key_prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoggingResult {
            bucket_name: o.get_field("bucketName"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            log_destination_type: o.get_field("logDestinationType"),
            log_exports: o.get_field("logExports"),
            s3_key_prefix: o.get_field("s3KeyPrefix"),
        }
    }
}
