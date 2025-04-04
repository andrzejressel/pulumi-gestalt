/// Provides a resource to create an organizational unit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organizational_unit::create(
///         "example",
///         OrganizationalUnitArgs::builder()
///             .name("example")
///             .parent_id("${exampleAwsOrganizationsOrganization.roots[0].id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Organizations Organizational Units using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:organizations/organizationalUnit:OrganizationalUnit example ou-1234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organizational_unit {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationalUnitArgs {
        /// The name for the organizational unit
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the parent organizational unit, which may be the root
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct OrganizationalUnitResult {
        /// List of child accounts for this Organizational Unit. Does not return account information for child Organizational Units. All elements have these attributes:
        pub accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::types::organizations::OrganizationalUnitAccount>,
        >,
        /// ARN of the organizational unit
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name for the organizational unit
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the parent organizational unit, which may be the root
        pub parent_id: pulumi_gestalt_rust::Output<String>,
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
        args: OrganizationalUnitArgs,
    ) -> OrganizationalUnitResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let parent_id_binding = args.parent_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:organizations/organizationalUnit:OrganizationalUnit".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationalUnitResult {
            accounts: o.get_field("accounts"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            parent_id: o.get_field("parentId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
