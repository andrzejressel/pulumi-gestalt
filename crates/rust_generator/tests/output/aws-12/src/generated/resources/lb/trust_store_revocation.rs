/// Provides a ELBv2 Trust Store Revocation for use with Application Load Balancer Listener resources.
///
/// ## Example Usage
///
/// ### Trust Store With Revocations
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = trust_store::create(
///         "test",
///         TrustStoreArgs::builder()
///             .ca_certificates_bundle_s_3_bucket("...")
///             .ca_certificates_bundle_s_3_key("...")
///             .name("tf-example-lb-ts")
///             .build_struct(),
///     );
///     let testTrustStoreRevocation = trust_store_revocation::create(
///         "testTrustStoreRevocation",
///         TrustStoreRevocationArgs::builder()
///             .revocations_s_3_bucket("...")
///             .revocations_s_3_key("...")
///             .trust_store_arn("${test.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Trust Store Revocations using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lb/trustStoreRevocation:TrustStoreRevocation example arn:aws:elasticloadbalancing:us-west-2:187416307283:truststore/my-trust-store/20cfe21448b66314,6
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust_store_revocation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreRevocationArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub revocations_s3_bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub revocations_s3_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub revocations_s3_object_version: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Trust Store ARN.
        #[builder(into)]
        pub trust_store_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrustStoreRevocationResult {
        /// AWS assigned RevocationId, (number).
        pub revocation_id: pulumi_gestalt_rust::Output<i32>,
        /// S3 Bucket name holding the client certificate CA bundle.
        pub revocations_s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        pub revocations_s3_key: pulumi_gestalt_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        pub revocations_s3_object_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Trust Store ARN.
        pub trust_store_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustStoreRevocationArgs,
    ) -> TrustStoreRevocationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let revocations_s3_bucket_binding = args
            .revocations_s3_bucket
            .get_output(context);
        let revocations_s3_key_binding = args.revocations_s3_key.get_output(context);
        let revocations_s3_object_version_binding = args
            .revocations_s3_object_version
            .get_output(context);
        let trust_store_arn_binding = args.trust_store_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lb/trustStoreRevocation:TrustStoreRevocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revocationsS3Bucket".into(),
                    value: &revocations_s3_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revocationsS3Key".into(),
                    value: &revocations_s3_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "revocationsS3ObjectVersion".into(),
                    value: &revocations_s3_object_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustStoreArn".into(),
                    value: &trust_store_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustStoreRevocationResult {
            revocation_id: o.get_field("revocationId"),
            revocations_s3_bucket: o.get_field("revocationsS3Bucket"),
            revocations_s3_key: o.get_field("revocationsS3Key"),
            revocations_s3_object_version: o.get_field("revocationsS3ObjectVersion"),
            trust_store_arn: o.get_field("trustStoreArn"),
        }
    }
}
