/// Three different resources help you manage your IAM policy for Healthcare DICOM store. Each of these resources serves a different use case:
///
/// * `gcp.healthcare.DicomStoreIamPolicy`: Authoritative. Sets the IAM policy for the DICOM store and replaces any existing policy already attached.
/// * `gcp.healthcare.DicomStoreIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the DICOM store are preserved.
/// * `gcp.healthcare.DicomStoreIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the DICOM store are preserved.
///
/// > **Note:** `gcp.healthcare.DicomStoreIamPolicy` **cannot** be used in conjunction with `gcp.healthcare.DicomStoreIamBinding` and `gcp.healthcare.DicomStoreIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.healthcare.DicomStoreIamBinding` resources **can be** used in conjunction with `gcp.healthcare.DicomStoreIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.healthcare.DicomStoreIamPolicy
///
/// ```yaml
/// resources:
///   dicomStore:
///     type: gcp:healthcare:DicomStoreIamPolicy
///     name: dicom_store
///     properties:
///       dicomStoreId: your-dicom-store-id
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/editor
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.healthcare.DicomStoreIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_binding::create(
///         "dicomStore",
///         DicomStoreIamBindingArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_member::create(
///         "dicomStore",
///         DicomStoreIamMemberArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_binding::create(
///         "dicomStore",
///         DicomStoreIamBindingArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.healthcare.DicomStoreIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dicomStore = dicom_store_iam_member::create(
///         "dicomStore",
///         DicomStoreIamMemberArgs::builder()
///             .dicom_store_id("your-dicom-store-id")
///             .member("user:jane@example.com")
///             .role("roles/editor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the Healthcare DICOM store resource. For example:
///
/// * `"{{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "{{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}"
///
///   to = google_healthcare_dicom_store_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStoreIamMember:DicomStoreIamMember default {{project_id}}/{{location}}/{{dataset}}/{{dicom_store}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dicom_store_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DicomStoreIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::DicomStoreIamMemberCondition>,
        >,
        /// The DICOM store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{dicom_store_name}` or
        /// `{location_name}/{dataset_name}/{dicom_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub dicom_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.DicomStoreIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DicomStoreIamMemberResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::DicomStoreIamMemberCondition>,
        >,
        /// The DICOM store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{dicom_store_name}` or
        /// `{location_name}/{dataset_name}/{dicom_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        pub dicom_store_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the DICOM store's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.healthcare.DicomStoreIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: DicomStoreIamMemberArgs,
    ) -> DicomStoreIamMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let dicom_store_id_binding = args.dicom_store_id.get_output(context);
        let member_binding = args.member.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/dicomStoreIamMember:DicomStoreIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dicomStoreId".into(),
                    value: &dicom_store_id_binding.drop_type(),
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
        DicomStoreIamMemberResult {
            condition: o.get_field("condition"),
            dicom_store_id: o.get_field("dicomStoreId"),
            etag: o.get_field("etag"),
            member: o.get_field("member"),
            role: o.get_field("role"),
        }
    }
}
