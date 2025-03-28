/// Three different resources help you manage your IAM policy for Compute Engine Image. Each of these resources serves a different use case:
///
/// * `gcp.compute.ImageIamPolicy`: Authoritative. Sets the IAM policy for the image and replaces any existing policy already attached.
/// * `gcp.compute.ImageIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the image are preserved.
/// * `gcp.compute.ImageIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the image are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.ImageIamPolicy`: Retrieves the IAM policy for the image
///
/// > **Note:** `gcp.compute.ImageIamPolicy` **cannot** be used in conjunction with `gcp.compute.ImageIamBinding` and `gcp.compute.ImageIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.ImageIamBinding` resources **can be** used in conjunction with `gcp.compute.ImageIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.compute.ImageIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:ImageIamPolicy
///     properties:
///       project: ${example.project}
///       image: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.imageUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:ImageIamPolicy
///     properties:
///       project: ${example.project}
///       image: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.imageUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.compute.ImageIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = image_iam_binding::create(
///         "binding",
///         ImageIamBindingArgs::builder()
///             .image("${example.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = image_iam_binding::create(
///         "binding",
///         ImageIamBindingArgs::builder()
///             .condition(
///                 ImageIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .image("${example.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.compute.ImageIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = image_iam_member::create(
///         "member",
///         ImageIamMemberArgs::builder()
///             .image("${example.name}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = image_iam_member::create(
///         "member",
///         ImageIamMemberArgs::builder()
///             .condition(
///                 ImageIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .image("${example.name}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Compute Engine Image
/// Three different resources help you manage your IAM policy for Compute Engine Image. Each of these resources serves a different use case:
///
/// * `gcp.compute.ImageIamPolicy`: Authoritative. Sets the IAM policy for the image and replaces any existing policy already attached.
/// * `gcp.compute.ImageIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the image are preserved.
/// * `gcp.compute.ImageIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the image are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.ImageIamPolicy`: Retrieves the IAM policy for the image
///
/// > **Note:** `gcp.compute.ImageIamPolicy` **cannot** be used in conjunction with `gcp.compute.ImageIamBinding` and `gcp.compute.ImageIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.ImageIamBinding` resources **can be** used in conjunction with `gcp.compute.ImageIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.compute.ImageIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:ImageIamPolicy
///     properties:
///       project: ${example.project}
///       image: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.imageUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:ImageIamPolicy
///     properties:
///       project: ${example.project}
///       image: ${example.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.imageUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.compute.ImageIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = image_iam_binding::create(
///         "binding",
///         ImageIamBindingArgs::builder()
///             .image("${example.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = image_iam_binding::create(
///         "binding",
///         ImageIamBindingArgs::builder()
///             .condition(
///                 ImageIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .image("${example.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.compute.ImageIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = image_iam_member::create(
///         "member",
///         ImageIamMemberArgs::builder()
///             .image("${example.name}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = image_iam_member::create(
///         "member",
///         ImageIamMemberArgs::builder()
///             .condition(
///                 ImageIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .image("${example.name}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/compute.imageUser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/global/images/{{name}}
///
/// * {{project}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Compute Engine image IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/imageIamPolicy:ImageIamPolicy editor "projects/{{project}}/global/images/{{image}} roles/compute.imageUser user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/imageIamPolicy:ImageIamPolicy editor "projects/{{project}}/global/images/{{image}} roles/compute.imageUser"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/imageIamPolicy:ImageIamPolicy editor projects/{{project}}/global/images/{{image}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ImageIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub image: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageIamPolicyArgs,
    ) -> ImageIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let image_binding = args.image.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/imageIamPolicy:ImageIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "image".into(),
                    value: &image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImageIamPolicyResult {
            etag: o.get_field("etag"),
            image: o.get_field("image"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
