/// Provides a CodeArtifact Domain Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder().domain("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Domain using the CodeArtifact Domain arn. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/domain:Domain example arn:aws:codeartifact:us-west-2:012345678912:domain/tf-acc-test-8593714120730241305
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// The name of the domain to create. All domain names in an AWS Region that are in the same AWS account must be unique. The domain name is used as the prefix in DNS hostnames. Do not use sensitive information in a domain name because it is publicly discoverable.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The encryption key for the domain. This is used to encrypt content stored in a domain. The KMS Key Amazon Resource Name (ARN). The default aws/codeartifact AWS KMS master key is used if this element is absent.
        #[builder(into, default)]
        pub encryption_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// The ARN of the Domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The total size of all assets in the domain.
        pub asset_size_bytes: pulumi_gestalt_rust::Output<String>,
        /// A timestamp that represents the date and time the domain was created in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the domain to create. All domain names in an AWS Region that are in the same AWS account must be unique. The domain name is used as the prefix in DNS hostnames. Do not use sensitive information in a domain name because it is publicly discoverable.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The encryption key for the domain. This is used to encrypt content stored in a domain. The KMS Key Amazon Resource Name (ARN). The default aws/codeartifact AWS KMS master key is used if this element is absent.
        pub encryption_key: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID that owns the domain.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The number of repositories in the domain.
        pub repository_count: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the Amazon S3 bucket that is used to store package assets in the domain.
        pub s3_bucket_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainArgs,
    ) -> DomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let encryption_key_binding = args.encryption_key.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codeartifact/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionKey".into(),
                    value: &encryption_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainResult {
            arn: o.get_field("arn"),
            asset_size_bytes: o.get_field("assetSizeBytes"),
            created_time: o.get_field("createdTime"),
            domain: o.get_field("domain"),
            encryption_key: o.get_field("encryptionKey"),
            owner: o.get_field("owner"),
            repository_count: o.get_field("repositoryCount"),
            s3_bucket_arn: o.get_field("s3BucketArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
