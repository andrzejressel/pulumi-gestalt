/// Provides a ELBv2 Trust Store for use with Application Load Balancer Listener resources.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Target Groups using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:lb/trustStore:TrustStore example arn:aws:elasticloadbalancing:us-west-2:187416307283:truststore/my-trust-store/20cfe21448b66314
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod trust_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustStoreArgs {
        /// S3 Bucket name holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_bucket: pulumi_gestalt_rust::Input<String>,
        /// S3 object key holding the client certificate CA bundle.
        #[builder(into)]
        pub ca_certificates_bundle_s3_key: pulumi_gestalt_rust::Input<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        #[builder(into, default)]
        pub ca_certificates_bundle_s3_object_version: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::Input<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustStoreResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Trust Store (matches `id`).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN suffix for use with CloudWatch Metrics.
        pub arn_suffix: pulumi_gestalt_rust::Output<String>,
        /// S3 Bucket name holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_bucket: pulumi_gestalt_rust::Output<String>,
        /// S3 object key holding the client certificate CA bundle.
        pub ca_certificates_bundle_s3_key: pulumi_gestalt_rust::Output<String>,
        /// Version Id of CA bundle S3 bucket object, if versioned, defaults to latest if omitted.
        pub ca_certificates_bundle_s3_object_version: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Name of the Trust Store. If omitted, the provider will assign a random, unique name. This name must be unique per region per account, can have a maximum of 32 characters, must contain only alphanumeric characters or hyphens, and must not begin or end with a hyphen.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Cannot be longer than 6 characters.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustStoreArgs,
    ) -> TrustStoreResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustStoreArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TrustStoreResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustStoreArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TrustStoreResult {
        let ca_certificates_bundle_s3_bucket_binding = args
            .ca_certificates_bundle_s3_bucket
            .get_output(ctx);
        let ca_certificates_bundle_s3_key_binding = args
            .ca_certificates_bundle_s3_key
            .get_output(ctx);
        let ca_certificates_bundle_s3_object_version_binding = args
            .ca_certificates_bundle_s3_object_version
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let name_prefix_binding = args.name_prefix.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lb/trustStore:TrustStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3Bucket".into(),
                    value: &ca_certificates_bundle_s3_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3Key".into(),
                    value: &ca_certificates_bundle_s3_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caCertificatesBundleS3ObjectVersion".into(),
                    value: &ca_certificates_bundle_s3_object_version_binding.drop_type(),
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
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TrustStoreResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            arn_suffix: o.get_field("arnSuffix"),
            ca_certificates_bundle_s3_bucket: o
                .get_field("caCertificatesBundleS3Bucket"),
            ca_certificates_bundle_s3_key: o.get_field("caCertificatesBundleS3Key"),
            ca_certificates_bundle_s3_object_version: o
                .get_field("caCertificatesBundleS3ObjectVersion"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
