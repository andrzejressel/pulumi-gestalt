/// Resource for managing an Amazon S3 Tables Table Bucket Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleTableBucketPolicy:
///     type: aws:s3tables:TableBucketPolicy
///     name: example
///     properties:
///       resourcePolicy: ${example.json}
///       tableBucketArn: ${exampleAwsS3tablesTableBucket.arn}
///   test:
///     type: aws:s3tables:TableBucket
///     properties:
///       name: example-bucket
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import S3 Tables Table Bucket Policy using the `table_bucket_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3tables/tableBucketPolicy:TableBucketPolicy example 'arn:aws:s3tables:us-west-2:123456789012:bucket/example-bucket;example-namespace'
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod table_bucket_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableBucketPolicyArgs {
        /// Amazon Web Services resource-based policy document in JSON format.
        #[builder(into)]
        pub resource_policy: pulumi_gestalt_rust::Input<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        #[builder(into)]
        pub table_bucket_arn: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct TableBucketPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Web Services resource-based policy document in JSON format.
        pub resource_policy: pulumi_gestalt_rust::Output<String>,
        /// ARN referencing the Table Bucket that owns this policy.
        pub table_bucket_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableBucketPolicyArgs,
    ) -> TableBucketPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableBucketPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TableBucketPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableBucketPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TableBucketPolicyResult {
        let resource_policy_binding = args.resource_policy.get_output(ctx);
        let table_bucket_arn_binding = args.table_bucket_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3tables/tableBucketPolicy:TableBucketPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePolicy".into(),
                    value: &resource_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableBucketArn".into(),
                    value: &table_bucket_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TableBucketPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            resource_policy: o.get_field("resourcePolicy"),
            table_bucket_arn: o.get_field("tableBucketArn"),
        }
    }
}
