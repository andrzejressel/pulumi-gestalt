/// Access Approval enables you to require your explicit approval whenever Google support and engineering need to access your customer content.
///
///
/// To get more information about OrganizationSettings, see:
///
/// * [API documentation](https://cloud.google.com/access-approval/docs/reference/rest/v1/organizations)
///
/// ## Example Usage
///
/// ### Organization Access Approval Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let organizationAccessApproval = access_approval_settings::create(
///         "organizationAccessApproval",
///         AccessApprovalSettingsArgs::builder()
///             .enrolled_services(
///                 vec![
///                     AccessApprovalSettingsEnrolledService::builder()
///                     .cloudProduct("appengine.googleapis.com").build_struct(),
///                     AccessApprovalSettingsEnrolledService::builder()
///                     .cloudProduct("dataflow.googleapis.com").enrollmentLevel("BLOCK_ALL")
///                     .build_struct(),
///                 ],
///             )
///             .notification_emails(
///                 vec!["testuser@example.com", "example.user@example.com",],
///             )
///             .organization_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Organization Access Approval Active Key Version
///
///
/// ```yaml
/// resources:
///   myProject:
///     type: gcp:organizations:Project
///     name: my_project
///     properties:
///       name: My Project
///       projectId: your-project-id
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: key-ring
///       location: global
///       project: ${myProject.projectId}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: crypto-key
///       keyRing: ${keyRing.id}
///       purpose: ASYMMETRIC_SIGN
///       versionTemplate:
///         algorithm: EC_SIGN_P384_SHA384
///   iam:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.signerVerifier
///       member: serviceAccount:${serviceAccount.accountEmail}
///   organizationAccessApproval:
///     type: gcp:organizations:AccessApprovalSettings
///     name: organization_access_approval
///     properties:
///       organizationId: '123456789'
///       activeKeyVersion: ${cryptoKeyVersion.name}
///       enrolledServices:
///         - cloudProduct: all
///     options:
///       dependsOn:
///         - ${iam}
/// variables:
///   serviceAccount:
///     fn::invoke:
///       function: gcp:accessapproval:getOrganizationServiceAccount
///       arguments:
///         organizationId: '123456789'
///   cryptoKeyVersion:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKeyVersion
///       arguments:
///         cryptoKey: ${cryptoKey.id}
/// ```
///
/// ## Import
///
/// OrganizationSettings can be imported using any of these accepted formats:
///
/// * `organizations/{{organization_id}}/accessApprovalSettings`
///
/// * `{{organization_id}}`
///
/// When using the `pulumi import` command, OrganizationSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:organizations/accessApprovalSettings:AccessApprovalSettings default organizations/{{organization_id}}/accessApprovalSettings
/// ```
///
/// ```sh
/// $ pulumi import gcp:organizations/accessApprovalSettings:AccessApprovalSettings default {{organization_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_approval_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessApprovalSettingsArgs {
        /// The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a
        /// Google-managed key should be used for signing.
        #[builder(into, default)]
        pub active_key_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of Google Cloud Services for which the given resource has Access Approval enrolled.
        /// Access requests for the resource given by name against any of these services contained here will be required
        /// to have explicit approval. Enrollment can be done for individual services.
        /// A maximum of 10 enrolled services will be enforced, to be expanded as the set of supported services is expanded.
        /// Structure is documented below.
        #[builder(into)]
        pub enrolled_services: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::organizations::AccessApprovalSettingsEnrolledService,
            >,
        >,
        /// A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to
        /// a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email
        /// addresses are allowed.
        #[builder(into, default)]
        pub notification_emails: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// ID of the organization of the access approval settings.
        #[builder(into)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessApprovalSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The asymmetric crypto key version to use for signing approval requests. Empty active_key_version indicates that a
        /// Google-managed key should be used for signing.
        pub active_key_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// This field will always be unset for the organization since organizations do not have ancestors.
        pub ancestor_has_active_key_version: pulumi_gestalt_rust::Output<bool>,
        /// This field will always be unset for the organization since organizations do not have ancestors.
        pub enrolled_ancestor: pulumi_gestalt_rust::Output<bool>,
        /// A list of Google Cloud Services for which the given resource has Access Approval enrolled.
        /// Access requests for the resource given by name against any of these services contained here will be required
        /// to have explicit approval. Enrollment can be done for individual services.
        /// A maximum of 10 enrolled services will be enforced, to be expanded as the set of supported services is expanded.
        /// Structure is documented below.
        pub enrolled_services: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::organizations::AccessApprovalSettingsEnrolledService,
            >,
        >,
        /// If the field is true, that indicates that there is some configuration issue with the active_key_version
        /// configured on this Organization (e.g. it doesn't exist or the Access Approval service account doesn't have the
        /// correct permissions on it, etc.).
        pub invalid_key_version: pulumi_gestalt_rust::Output<bool>,
        /// The resource name of the settings. Format is "organizations/{organization_id}/accessApprovalSettings"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of email addresses to which notifications relating to approval requests should be sent. Notifications relating to
        /// a resource will be sent to all emails in the settings of ancestor resources of that resource. A maximum of 50 email
        /// addresses are allowed.
        pub notification_emails: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the organization of the access approval settings.
        pub organization_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessApprovalSettingsArgs,
    ) -> AccessApprovalSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let active_key_version_binding = args.active_key_version.get_output(context);
        let enrolled_services_binding = args.enrolled_services.get_output(context);
        let notification_emails_binding = args.notification_emails.get_output(context);
        let organization_id_binding = args.organization_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/accessApprovalSettings:AccessApprovalSettings"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activeKeyVersion".into(),
                    value: &active_key_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enrolledServices".into(),
                    value: &enrolled_services_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationEmails".into(),
                    value: &notification_emails_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessApprovalSettingsResult {
            id: o.get_field("id"),
            active_key_version: o.get_field("activeKeyVersion"),
            ancestor_has_active_key_version: o.get_field("ancestorHasActiveKeyVersion"),
            enrolled_ancestor: o.get_field("enrolledAncestor"),
            enrolled_services: o.get_field("enrolledServices"),
            invalid_key_version: o.get_field("invalidKeyVersion"),
            name: o.get_field("name"),
            notification_emails: o.get_field("notificationEmails"),
            organization_id: o.get_field("organizationId"),
        }
    }
}
