/// Manages S3 account-level Public Access Block configuration. For more information about these settings, see the [AWS S3 Block Public Access documentation](https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html).
///
/// > **NOTE:** Each AWS account may only have one S3 Public Access Block configuration. Multiple configurations of the resource against the same AWS account will cause a perpetual difference.
///
/// > Advanced usage: To use a custom API endpoint for this resource, use the `s3control` endpoint provider configuration, not the `s3` endpoint provider configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_public_access_block::create(
///         "example",
///         AccountPublicAccessBlockArgs::builder()
///             .block_public_acls(true)
///             .block_public_policy(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_s3_account_public_access_block` using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:s3/accountPublicAccessBlock:AccountPublicAccessBlock example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_public_access_block {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountPublicAccessBlockArgs {
        /// AWS account ID to configure. Defaults to automatically determined account ID of the this provider AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether Amazon S3 should block public ACLs for buckets in this account. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls fail if the request includes a public ACL.
        #[builder(into, default)]
        pub block_public_acls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for buckets in this account. Defaults to `false`. Enabling this setting does not affect existing bucket policies. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        #[builder(into, default)]
        pub block_public_policy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Amazon S3 should ignore public ACLs for buckets in this account. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore all public ACLs on buckets in this account and any objects that they contain.
        #[builder(into, default)]
        pub ignore_public_acls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for buckets in this account. Defaults to `false`. Enabling this setting does not affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access buckets with public policies.
        #[builder(into, default)]
        pub restrict_public_buckets: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AccountPublicAccessBlockResult {
        /// AWS account ID to configure. Defaults to automatically determined account ID of the this provider AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether Amazon S3 should block public ACLs for buckets in this account. Defaults to `false`. Enabling this setting does not affect existing policies or ACLs. When set to `true` causes the following behavior:
        /// * PUT Bucket acl and PUT Object acl calls will fail if the specified ACL allows public access.
        /// * PUT Object calls fail if the request includes a public ACL.
        pub block_public_acls: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should block public bucket policies for buckets in this account. Defaults to `false`. Enabling this setting does not affect existing bucket policies. When set to `true` causes Amazon S3 to:
        /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
        pub block_public_policy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should ignore public ACLs for buckets in this account. Defaults to `false`. Enabling this setting does not affect the persistence of any existing ACLs and doesn't prevent new public ACLs from being set. When set to `true` causes Amazon S3 to:
        /// * Ignore all public ACLs on buckets in this account and any objects that they contain.
        pub ignore_public_acls: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether Amazon S3 should restrict public bucket policies for buckets in this account. Defaults to `false`. Enabling this setting does not affect previously stored bucket policies, except that public and cross-account access within any public bucket policy, including non-public delegation to specific accounts, is blocked. When set to `true`:
        /// * Only the bucket owner and AWS Services can access buckets with public policies.
        pub restrict_public_buckets: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountPublicAccessBlockArgs,
    ) -> AccountPublicAccessBlockResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let block_public_acls_binding = args.block_public_acls.get_output(context);
        let block_public_policy_binding = args.block_public_policy.get_output(context);
        let ignore_public_acls_binding = args.ignore_public_acls.get_output(context);
        let restrict_public_buckets_binding = args
            .restrict_public_buckets
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3/accountPublicAccessBlock:AccountPublicAccessBlock".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicAcls".into(),
                    value: &block_public_acls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockPublicPolicy".into(),
                    value: &block_public_policy_binding.drop_type(),
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
        AccountPublicAccessBlockResult {
            account_id: o.get_field("accountId"),
            block_public_acls: o.get_field("blockPublicAcls"),
            block_public_policy: o.get_field("blockPublicPolicy"),
            ignore_public_acls: o.get_field("ignorePublicAcls"),
            restrict_public_buckets: o.get_field("restrictPublicBuckets"),
        }
    }
}
