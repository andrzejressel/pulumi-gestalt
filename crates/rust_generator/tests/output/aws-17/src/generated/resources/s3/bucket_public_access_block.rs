/// Manages S3 bucket-level Public Access Block configuration. For more information about these settings, see the [AWS S3 Block Public Access documentation](https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html).
///
/// > This resource cannot be used with S3 directory buckets.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = bucket_v_2::create(
///         "example",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleBucketPublicAccessBlock = bucket_public_access_block::create(
///         "exampleBucketPublicAccessBlock",
///         BucketPublicAccessBlockArgs::builder()
///             .block_public_acls(true)
///             .block_public_policy(true)
///             .bucket("${example.id}")
///             .ignore_public_acls(true)
///             .restrict_public_buckets(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_s3_bucket_public_access_block` using the bucket name. For example:
///
/// ```sh
/// $ pulumi import aws:s3/bucketPublicAccessBlock:BucketPublicAccessBlock example my-bucket
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_public_access_block {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketPublicAccessBlockArgs {
        /// Whether Amazon S3 should block public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls will fail if the request includes an object ACL.
        #[builder(into, default)]
        pub block_public_acls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the existing bucket policy. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        #[builder(into, default)]
        pub block_public_policy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// S3 Bucket to which this Public Access Block configuration should be applied.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether Amazon S3 should ignore public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore public ACLs on this bucket and any objects that it contains.
        #[builder(into, default)]
        pub ignore_public_acls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the previously stored bucket policy, except that public and cross-account access within the public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access this buckets if it has a public policy.
        #[builder(into, default)]
        pub restrict_public_buckets: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct BucketPublicAccessBlockResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether Amazon S3 should block public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls will fail if the request includes an object ACL.
        pub block_public_acls: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the existing bucket policy. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        pub block_public_policy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// S3 Bucket to which this Public Access Block configuration should be applied.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Whether Amazon S3 should ignore public ACLs for this bucket. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore public ACLs on this bucket and any objects that it contains.
        pub ignore_public_acls: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for this bucket. Defaults to `false`. Enabling this setting does not affect the previously stored bucket policy, except that public and cross-account access within the public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access this buckets if it has a public policy.
        pub restrict_public_buckets: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketPublicAccessBlockArgs,
    ) -> BucketPublicAccessBlockResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_public_acls_binding = args.block_public_acls.get_output(context);
        let block_public_policy_binding = args.block_public_policy.get_output(context);
        let bucket_binding = args.bucket.get_output(context);
        let ignore_public_acls_binding = args.ignore_public_acls.get_output(context);
        let restrict_public_buckets_binding = args
            .restrict_public_buckets
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/bucketPublicAccessBlock:BucketPublicAccessBlock".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicAcls".into(),
                    value: &block_public_acls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicPolicy".into(),
                    value: &block_public_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignorePublicAcls".into(),
                    value: &ignore_public_acls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restrictPublicBuckets".into(),
                    value: &restrict_public_buckets_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketPublicAccessBlockResult {
            id: o.get_field("id"),
            block_public_acls: o.get_field("blockPublicAcls"),
            block_public_policy: o.get_field("blockPublicPolicy"),
            bucket: o.get_field("bucket"),
            ignore_public_acls: o.get_field("ignorePublicAcls"),
            restrict_public_buckets: o.get_field("restrictPublicBuckets"),
        }
    }
}
