/// The default version for the processor. Deleting this resource is a no-op, and does not unset the default version.
///
///
///
/// ## Example Usage
///
/// ### Documentai Default Version
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let processor = document_ai_processor::create(
///         "processor",
///         DocumentAiProcessorArgs::builder()
///             .display_name("test-processor")
///             .location("us")
///             .type_("OCR_PROCESSOR")
///             .build_struct(),
///     );
///     let processorDocumentAiProcessorDefaultVersion = document_ai_processor_default_version::create(
///         "processorDocumentAiProcessorDefaultVersion",
///         DocumentAiProcessorDefaultVersionArgs::builder()
///             .processor("${processor.id}")
///             .version("${processor.id}/processorVersions/stable")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ProcessorDefaultVersion can be imported using any of these accepted formats:
///
/// * `{{processor}}`
///
/// When using the `pulumi import` command, ProcessorDefaultVersion can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:essentialcontacts/documentAiProcessorDefaultVersion:DocumentAiProcessorDefaultVersion default {{processor}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod document_ai_processor_default_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentAiProcessorDefaultVersionArgs {
        /// The processor to set the version on.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub processor: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The version to set. Using `stable` or `rc` will cause the API to return the latest version in that release channel.
        /// Apply `lifecycle.ignore_changes` to the `version` field to suppress this diff.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DocumentAiProcessorDefaultVersionResult {
        /// The processor to set the version on.
        ///
        ///
        /// - - -
        pub processor: pulumi_gestalt_rust::Output<String>,
        /// The version to set. Using `stable` or `rc` will cause the API to return the latest version in that release channel.
        /// Apply `lifecycle.ignore_changes` to the `version` field to suppress this diff.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DocumentAiProcessorDefaultVersionArgs,
    ) -> DocumentAiProcessorDefaultVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let processor_binding = args.processor.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:essentialcontacts/documentAiProcessorDefaultVersion:DocumentAiProcessorDefaultVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "processor".into(),
                    value: &processor_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DocumentAiProcessorDefaultVersionResult {
            processor: o.get_field("processor"),
            version: o.get_field("version"),
        }
    }
}
