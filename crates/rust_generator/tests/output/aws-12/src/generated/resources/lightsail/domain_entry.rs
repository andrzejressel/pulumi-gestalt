/// Creates a domain entry resource
///
/// > **NOTE on `id`:** In an effort to simplify imports, this resource `id` field has been updated to the standard resource id separator, a comma (`,`). For backward compatibility, the previous separator (underscore `_`) can still be used to read and import existing resources. When state is refreshed, the `id` will be updated to use the new standard separator. The previous separator will be deprecated in a future major release.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = domain::create(
///         "test",
///         DomainArgs::builder().domain_name("mydomain.com").build_struct(),
///     );
///     let testDomainEntry = domain_entry::create(
///         "testDomainEntry",
///         DomainEntryArgs::builder()
///             .domain_name("${domainTest.domainName}")
///             .name("www")
///             .target("127.0.0.1")
///             .type_("A")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_domain_entry` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/domainEntry:DomainEntry example www,mydomain.com,A,127.0.0.1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_entry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainEntryArgs {
        /// The name of the Lightsail domain in which to create the entry
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If the entry should be an alias Defaults to `false`
        #[builder(into, default)]
        pub is_alias: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the entry record
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Target of the domain entry
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of record
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainEntryResult {
        /// The name of the Lightsail domain in which to create the entry
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// If the entry should be an alias Defaults to `false`
        pub is_alias: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the entry record
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Target of the domain entry
        pub target: pulumi_gestalt_rust::Output<String>,
        /// Type of record
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainEntryArgs,
    ) -> DomainEntryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let is_alias_binding = args.is_alias.get_output(context);
        let name_binding = args.name.get_output(context);
        let target_binding = args.target.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/domainEntry:DomainEntry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isAlias".into(),
                    value: &is_alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainEntryResult {
            domain_name: o.get_field("domainName"),
            is_alias: o.get_field("isAlias"),
            name: o.get_field("name"),
            target: o.get_field("target"),
            type_: o.get_field("type"),
        }
    }
}
