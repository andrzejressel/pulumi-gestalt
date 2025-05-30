/// Resource for managing an AWS DevOps Guru Resource Collection.
///
/// > Only one type of resource collection (All Account Resources, CloudFormation, or Tags) can be enabled in an account at a time. To avoid persistent differences, this resource should be defined only once.
///
/// ## Example Usage
///
/// ### All Account Resources
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .cloudformation(
///                 ResourceCollectionCloudformation::builder()
///                     .stackNames(vec!["*",])
///                     .build_struct(),
///             )
///             .type_("AWS_SERVICE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### CloudFormation Stacks
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .cloudformation(
///                 ResourceCollectionCloudformation::builder()
///                     .stackNames(vec!["ExampleStack",])
///                     .build_struct(),
///             )
///             .type_("AWS_CLOUD_FORMATION")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tags
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .tags(
///                 ResourceCollectionTags::builder()
///                     .appBoundaryKey("DevOps-Guru-Example")
///                     .tagValues(vec!["Example-Value",])
///                     .build_struct(),
///             )
///             .type_("AWS_TAGS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tags All Resources
///
/// To analyze all resources with the `app_boundary_key` regardless of the corresponding tag value, set `tag_values` to `["*"]`.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .tags(
///                 ResourceCollectionTags::builder()
///                     .appBoundaryKey("DevOps-Guru-Example")
///                     .tagValues(vec!["*",])
///                     .build_struct(),
///             )
///             .type_("AWS_TAGS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Resource Collection using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/resourceCollection:ResourceCollection example AWS_CLOUD_FORMATION
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceCollectionArgs {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        #[builder(into, default)]
        pub cloudformation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devopsguru::ResourceCollectionCloudformation>,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devopsguru::ResourceCollectionTags>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceCollectionResult {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        pub cloudformation: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionCloudformation>,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        pub tags: pulumi_gestalt_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionTags>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourceCollectionArgs,
    ) -> ResourceCollectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloudformation_binding = args.cloudformation.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devopsguru/resourceCollection:ResourceCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudformation".into(),
                    value: &cloudformation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceCollectionResult {
            cloudformation: o.get_field("cloudformation"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
