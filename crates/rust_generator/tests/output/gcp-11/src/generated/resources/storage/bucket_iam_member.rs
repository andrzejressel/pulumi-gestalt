/// Three different resources help you manage your IAM policy for Cloud Storage Bucket. Each of these resources serves a different use case:
///
/// * `gcp.storage.BucketIAMPolicy`: Authoritative. Sets the IAM policy for the bucket and replaces any existing policy already attached.
/// * `gcp.storage.BucketIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the bucket are preserved.
/// * `gcp.storage.BucketIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the bucket are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.storage.BucketIAMPolicy`: Retrieves the IAM policy for the bucket
///
/// > **Note:** `gcp.storage.BucketIAMPolicy` **cannot** be used in conjunction with `gcp.storage.BucketIAMBinding` and `gcp.storage.BucketIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.storage.BucketIAMBinding` resources **can be** used in conjunction with `gcp.storage.BucketIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.storage.BucketIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:BucketIAMPolicy
///     properties:
///       bucket: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:BucketIAMPolicy
///     properties:
///       bucket: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.storage.BucketIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = bucket_iam_binding::create(
///         "binding",
///         BucketIamBindingArgs::builder()
///             .bucket("${default.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
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
///     let binding = bucket_iam_binding::create(
///         "binding",
///         BucketIamBindingArgs::builder()
///             .bucket("${default.name}")
///             .condition(
///                 BucketIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.storage.BucketIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = bucket_iam_member::create(
///         "member",
///         BucketIamMemberArgs::builder()
///             .bucket("${default.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
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
///     let member = bucket_iam_member::create(
///         "member",
///         BucketIamMemberArgs::builder()
///             .bucket("${default.name}")
///             .condition(
///                 BucketIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Cloud Storage Bucket
/// Three different resources help you manage your IAM policy for Cloud Storage Bucket. Each of these resources serves a different use case:
///
/// * `gcp.storage.BucketIAMPolicy`: Authoritative. Sets the IAM policy for the bucket and replaces any existing policy already attached.
/// * `gcp.storage.BucketIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the bucket are preserved.
/// * `gcp.storage.BucketIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the bucket are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.storage.BucketIAMPolicy`: Retrieves the IAM policy for the bucket
///
/// > **Note:** `gcp.storage.BucketIAMPolicy` **cannot** be used in conjunction with `gcp.storage.BucketIAMBinding` and `gcp.storage.BucketIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.storage.BucketIAMBinding` resources **can be** used in conjunction with `gcp.storage.BucketIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.storage.BucketIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:BucketIAMPolicy
///     properties:
///       bucket: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:storage:BucketIAMPolicy
///     properties:
///       bucket: ${default.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/storage.admin
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.storage.BucketIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = bucket_iam_binding::create(
///         "binding",
///         BucketIamBindingArgs::builder()
///             .bucket("${default.name}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
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
///     let binding = bucket_iam_binding::create(
///         "binding",
///         BucketIamBindingArgs::builder()
///             .bucket("${default.name}")
///             .condition(
///                 BucketIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.storage.BucketIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = bucket_iam_member::create(
///         "member",
///         BucketIamMemberArgs::builder()
///             .bucket("${default.name}")
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
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
///     let member = bucket_iam_member::create(
///         "member",
///         BucketIamMemberArgs::builder()
///             .bucket("${default.name}")
///             .condition(
///                 BucketIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .role("roles/storage.admin")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * b/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Storage bucket IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/bucketIAMMember:BucketIAMMember editor "b/{{bucket}} roles/storage.objectViewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/bucketIAMMember:BucketIAMMember editor "b/{{bucket}} roles/storage.objectViewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:storage/bucketIAMMember:BucketIAMMember editor b/{{bucket}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketIAMMemberArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketIamMemberCondition>,
        >,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.storage.BucketIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BucketIAMMemberResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding.
        /// Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.storage.BucketIAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketIAMMemberArgs,
    ) -> BucketIAMMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let condition_binding = args.condition.get_output(context);
        let member_binding = args.member.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/bucketIAMMember:BucketIAMMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketIAMMemberResult {
            id: o.get_field("id"),
            bucket: o.get_field("bucket"),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            member: o.get_field("member"),
            role: o.get_field("role"),
        }
    }
}
