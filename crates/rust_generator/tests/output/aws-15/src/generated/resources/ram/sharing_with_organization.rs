/// Manages Resource Access Manager (RAM) Resource Sharing with AWS Organizations. If you enable sharing with your organization, you can share resources without using invitations. Refer to the [AWS RAM user guide](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs) for more details.
///
/// > **NOTE:** Use this resource to manage resource sharing within your organization, **not** the `aws.organizations.Organization` resource with `ram.amazonaws.com` configured in `aws_service_access_principals`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = sharing_with_organization::create(
///         "example",
///         SharingWithOrganizationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the resource using the current AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:ram/sharingWithOrganization:SharingWithOrganization example 123456789012
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod sharing_with_organization {
    #[allow(dead_code)]
    pub struct SharingWithOrganizationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
    ) -> SharingWithOrganizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ram/sharingWithOrganization:SharingWithOrganization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[],
        };
        let o = context.register_resource(request);
        SharingWithOrganizationResult {
            id: o.get_field("id"),
        }
    }
}
