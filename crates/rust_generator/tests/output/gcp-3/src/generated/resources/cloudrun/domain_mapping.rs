/// Resource to hold the state and status of a user's domain mapping.
///
///
/// To get more information about DomainMapping, see:
///
/// * [API documentation](https://cloud.google.com/run/docs/reference/rest/v1/projects.locations.domainmappings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/run/docs/mapping-custom-domains)
///
/// ## Example Usage
///
/// ### Cloud Run Domain Mapping Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .location("us-central1")
///             .metadata(
///                 ServiceMetadata::builder().namespace("my-project-name").build_struct(),
///             )
///             .name("cloudrun-srv")
///             .template(
///                 ServiceTemplate::builder()
///                     .spec(
///                         ServiceTemplateSpec::builder()
///                             .containers(
///                                 vec![
///                                     ServiceTemplateSpecContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/hello")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let defaultDomainMapping = domain_mapping::create(
///         "defaultDomainMapping",
///         DomainMappingArgs::builder()
///             .location("us-central1")
///             .metadata(
///                 DomainMappingMetadata::builder()
///                     .namespace("my-project-name")
///                     .build_struct(),
///             )
///             .name("verified-domain.com")
///             .spec(
///                 DomainMappingSpec::builder().routeName("${default.name}").build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DomainMapping can be imported using any of these accepted formats:
///
/// * `locations/{{location}}/namespaces/{{project}}/domainmappings/{{name}}`
///
/// * `{{location}}/{{project}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, DomainMapping can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudrun/domainMapping:DomainMapping default locations/{{location}}/namespaces/{{project}}/domainmappings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrun/domainMapping:DomainMapping default {{location}}/{{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrun/domainMapping:DomainMapping default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainMappingArgs {
        /// The location of the cloud run instance. eg us-central1
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata associated with this DomainMapping.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudrun::DomainMappingMetadata>,
        >,
        /// Name should be a [verified](https://support.google.com/webmasters/answer/9008080) domain
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The spec for this DomainMapping.
        /// Structure is documented below.
        #[builder(into)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudrun::DomainMappingSpec,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainMappingResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location of the cloud run instance. eg us-central1
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Metadata associated with this DomainMapping.
        pub metadata: pulumi_gestalt_rust::Output<
            super::super::types::cloudrun::DomainMappingMetadata,
        >,
        /// Name should be a [verified](https://support.google.com/webmasters/answer/9008080) domain
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The spec for this DomainMapping.
        /// Structure is documented below.
        pub spec: pulumi_gestalt_rust::Output<
            super::super::types::cloudrun::DomainMappingSpec,
        >,
        /// (Output)
        /// Status of the condition, one of True, False, Unknown.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrun::DomainMappingStatus>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainMappingArgs,
    ) -> DomainMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let spec_binding = args.spec.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudrun/domainMapping:DomainMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spec".into(),
                    value: &spec_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainMappingResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            spec: o.get_field("spec"),
            statuses: o.get_field("statuses"),
        }
    }
}
