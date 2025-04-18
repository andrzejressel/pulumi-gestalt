/// Creates an Amazon CloudFront origin access identity.
///
/// For information about CloudFront distributions, see the
/// [Amazon CloudFront Developer Guide](http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html). For more information on generating
/// origin access identities, see
/// [Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content][2].
///
/// ## Example Usage
///
/// The following example below creates a CloudFront origin access identity.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = origin_access_identity::create(
///         "example",
///         OriginAccessIdentityArgs::builder().comment("Some comment").build_struct(),
///     );
/// }
/// ```
///
/// ## Using With CloudFront
///
/// Normally, when referencing an origin access identity in CloudFront, you need to
/// prefix the ID with the `origin-access-identity/cloudfront/` special path.
/// The `cloudfront_access_identity_path` allows this to be circumvented.
/// The below snippet demonstrates use with the `s3_origin_config` structure for the
/// `aws.cloudfront.Distribution` resource:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = distribution::create(
///         "example",
///         DistributionArgs::builder()
///             .origins(
///                 vec![
///                     DistributionOrigin::builder()
///                     .s3OriginConfig(DistributionOriginS3OriginConfig::builder()
///                     .originAccessIdentity("${exampleAwsCloudfrontOriginAccessIdentity.cloudfrontAccessIdentityPath}")
///                     .build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Updating your bucket policy
///
/// Note that the AWS API may translate the `s3_canonical_user_id` `CanonicalUser`
/// principal into an `AWS` IAM ARN principal when supplied in an
/// `aws.s3.BucketV2` bucket policy, causing spurious diffs. If
/// you see this behavior, use the `iam_arn` instead:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketPolicy
///     properties:
///       bucket: ${exampleAwsS3Bucket.id}
///       policy: ${s3Policy.json}
/// variables:
///   s3Policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - s3:GetObject
///             resources:
///               - ${exampleAwsS3Bucket.arn}/*
///             principals:
///               - type: AWS
///                 identifiers:
///                   - ${exampleAwsCloudfrontOriginAccessIdentity.iamArn}
/// ```
///
/// [1]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html
/// [2]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Origin Access Identities using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/originAccessIdentity:OriginAccessIdentity origin_access E74FTE3AEXAMPLE
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod origin_access_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginAccessIdentityArgs {
        /// An optional comment for the origin access identity.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OriginAccessIdentityResult {
        /// Internal value used by CloudFront to allow future
        /// updates to the origin access identity.
        pub caller_reference: pulumi_gestalt_rust::Output<String>,
        /// A shortcut to the full path for the
        /// origin access identity to use in CloudFront, see below.
        pub cloudfront_access_identity_path: pulumi_gestalt_rust::Output<String>,
        /// An optional comment for the origin access identity.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current version of the origin access identity's information.
        /// For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A pre-generated ARN for use in S3 bucket policies (see below).
        /// Example: `arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity
        /// E2QWRUHAPOMQZL`.
        pub iam_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon S3 canonical user ID for the origin
        /// access identity, which you use when giving the origin access identity read
        /// permission to an object in Amazon S3.
        pub s3_canonical_user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OriginAccessIdentityArgs,
    ) -> OriginAccessIdentityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/originAccessIdentity:OriginAccessIdentity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OriginAccessIdentityResult {
            caller_reference: o.get_field("callerReference"),
            cloudfront_access_identity_path: o.get_field("cloudfrontAccessIdentityPath"),
            comment: o.get_field("comment"),
            etag: o.get_field("etag"),
            iam_arn: o.get_field("iamArn"),
            s3_canonical_user_id: o.get_field("s3CanonicalUserId"),
        }
    }
}
