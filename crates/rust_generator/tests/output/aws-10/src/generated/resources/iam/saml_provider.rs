/// Provides an IAM SAML provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:iam:SamlProvider
///     properties:
///       name: myprovider
///       samlMetadataDocument:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: saml-metadata.xml
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM SAML Providers using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/samlProvider:SamlProvider default arn:aws:iam::123456789012:saml-provider/SAMLADFS
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod saml_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SamlProviderArgs {
        /// The name of the provider to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An XML document generated by an identity provider that supports SAML 2.0.
        #[builder(into)]
        pub saml_metadata_document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of resource tags for the IAM SAML provider. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SamlProviderResult {
        /// The ARN assigned by AWS for this provider.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the provider to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An XML document generated by an identity provider that supports SAML 2.0.
        pub saml_metadata_document: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags for the IAM SAML provider. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The expiration date and time for the SAML provider in RFC1123 format, e.g., `Mon, 02 Jan 2006 15:04:05 MST`.
        pub valid_until: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SamlProviderArgs,
    ) -> SamlProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let saml_metadata_document_binding = args
            .saml_metadata_document
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/samlProvider:SamlProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samlMetadataDocument".into(),
                    value: &saml_metadata_document_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SamlProviderResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            saml_metadata_document: o.get_field("samlMetadataDocument"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            valid_until: o.get_field("validUntil"),
        }
    }
}
