/// Provides a S3 bucket [metrics configuration](http://docs.aws.amazon.com/AmazonS3/latest/dev/metrics-configurations.html) resource.
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ### Add metrics configuration for entire S3 bucket
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   example-entire-bucket:
///     type: aws:s3:BucketMetric
///     properties:
///       bucket: ${example.id}
///       name: EntireBucket
/// ```
///
/// ### Add metrics configuration with S3 object filter
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   example-filtered:
///     type: aws:s3:BucketMetric
///     properties:
///       bucket: ${example.id}
///       name: ImportantBlueDocuments
///       filter:
///         prefix: documents/
///         tags:
///           priority: high
///           class: blue
/// ```
///
/// ### Add metrics configuration with S3 object filter for S3 Access Point
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   example-access-point:
///     type: aws:s3:AccessPoint
///     properties:
///       bucket: ${example.id}
///       name: example-access-point
///   example-filtered:
///     type: aws:s3:BucketMetric
///     properties:
///       bucket: ${example.id}
///       name: ImportantBlueDocuments
///       filter:
///         accessPoint: ${["example-access-point"].arn}
///         tags:
///           priority: high
///           class: blue
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 bucket metric configurations using `bucket:metric`. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketMetric:BucketMetric my-bucket-entire-bucket my-bucket:EntireBucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_metric {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketMetricArgs {
        /// Name of the bucket to put metric configuration.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// [Object filtering](http://docs.aws.amazon.com/AmazonS3/latest/dev/metrics-configurations.html#metrics-configurations-filter) that accepts a prefix, tags, or a logical AND of prefix and tags (documented below).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::s3::BucketMetricFilter>,
        >,
        /// Unique identifier of the metrics configuration for the bucket. Must be less than or equal to 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketMetricResult {
        /// Name of the bucket to put metric configuration.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// [Object filtering](http://docs.aws.amazon.com/AmazonS3/latest/dev/metrics-configurations.html#metrics-configurations-filter) that accepts a prefix, tags, or a logical AND of prefix and tags (documented below).
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::s3::BucketMetricFilter>,
        >,
        /// Unique identifier of the metrics configuration for the bucket. Must be less than or equal to 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketMetricArgs,
    ) -> BucketMetricResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketMetric:BucketMetric".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketMetricResult {
            bucket: o.get_field("bucket"),
            filter: o.get_field("filter"),
            name: o.get_field("name"),
        }
    }
}
