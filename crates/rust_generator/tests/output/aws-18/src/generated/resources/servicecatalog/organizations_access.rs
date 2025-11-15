/// Manages Service Catalog AWS Organizations Access, a portfolio sharing feature through AWS Organizations. This allows Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This resource will prompt AWS to set `organizations:EnableAWSServiceAccess` on your behalf so that your shares can be in sync with any changes in your AWS Organizations structure.
///
/// > **NOTE:** This resource can only be used by the management account in the organization. In other words, a delegated administrator is not authorized to use the resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:OrganizationsAccess
///     properties:
///       enabled: 'true'
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod organizations_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationsAccessArgs {
        /// Whether to enable AWS Organizations access.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct OrganizationsAccessResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable AWS Organizations access.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationsAccessArgs,
    ) -> OrganizationsAccessResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/organizationsAccess:OrganizationsAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationsAccessResult {
            id: o.get_field("id"),
            enabled: o.get_field("enabled"),
        }
    }
}
