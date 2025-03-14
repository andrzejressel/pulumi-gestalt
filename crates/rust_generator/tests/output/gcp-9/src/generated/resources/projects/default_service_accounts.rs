/// Allows management of Google Cloud Platform project default service accounts.
///
/// When certain service APIs are enabled, Google Cloud Platform automatically creates service accounts to help get started, but
/// this is not recommended for production environments as per [Google's documentation](https://cloud.google.com/iam/docs/service-accounts#default).
/// See the [Organization documentation](https://cloud.google.com/resource-manager/docs/quickstarts) for more details.
///
/// > **WARNING** Some Google Cloud products do not work if the default service accounts are deleted so it is better to `DEPRIVILEGE` as
/// Google **CAN NOT** recover service accounts that have been deleted for more than 30 days.
/// Also Google recommends using the `constraints/iam.automaticIamGrantsForDefaultServiceAccounts` [constraint](https://www.terraform.io/docs/providers/google/r/google_organization_policy.html)
/// to disable automatic IAM Grants to default service accounts.
///
/// > This resource works on a best-effort basis, as no API formally describes the default service accounts
/// and it is for users who are unable to use constraints. If the default service accounts change their name
/// or additional service accounts are added, this resource will need to be updated.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProject = default_service_accounts::create(
///         "myProject",
///         DefaultServiceAccountsArgs::builder()
///             .action("DELETE")
///             .project("my-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To enable the default service accounts on the resource destroy:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProject = default_service_accounts::create(
///         "myProject",
///         DefaultServiceAccountsArgs::builder()
///             .action("DISABLE")
///             .project("my-project-id")
///             .restore_policy("REVERT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// This resource does not support import
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_service_accounts {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultServiceAccountsArgs {
        /// The action to be performed in the default service accounts. Valid values are: `DEPRIVILEGE`, `DELETE`, `DISABLE`. Note that `DEPRIVILEGE` action will ignore the REVERT configuration in the restore_policy
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project ID where service accounts are created.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The action to be performed in the default service accounts on the resource destroy.
        /// Valid values are NONE, REVERT and REVERT_AND_IGNORE_FAILURE. It is applied for any action but in the DEPRIVILEGE.
        /// If set to REVERT it attempts to restore all default SAs but the DEPRIVILEGE action.
        /// If set to REVERT_AND_IGNORE_FAILURE it is the same behavior as REVERT but ignores errors returned by the API.
        #[builder(into, default)]
        pub restore_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DefaultServiceAccountsResult {
        /// The action to be performed in the default service accounts. Valid values are: `DEPRIVILEGE`, `DELETE`, `DISABLE`. Note that `DEPRIVILEGE` action will ignore the REVERT configuration in the restore_policy
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The project ID where service accounts are created.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The action to be performed in the default service accounts on the resource destroy.
        /// Valid values are NONE, REVERT and REVERT_AND_IGNORE_FAILURE. It is applied for any action but in the DEPRIVILEGE.
        /// If set to REVERT it attempts to restore all default SAs but the DEPRIVILEGE action.
        /// If set to REVERT_AND_IGNORE_FAILURE it is the same behavior as REVERT but ignores errors returned by the API.
        pub restore_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Service Accounts changed by this resource. It is used for `REVERT` the `action` on the destroy.
        pub service_accounts: pulumi_gestalt_rust::Output<
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
        args: DefaultServiceAccountsArgs,
    ) -> DefaultServiceAccountsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let action_binding = args.action.get_output(context);
        let project_binding = args.project.get_output(context);
        let restore_policy_binding = args.restore_policy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:projects/defaultServiceAccounts:DefaultServiceAccounts".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: &action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restorePolicy".into(),
                    value: &restore_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultServiceAccountsResult {
            action: o.get_field("action"),
            project: o.get_field("project"),
            restore_policy: o.get_field("restorePolicy"),
            service_accounts: o.get_field("serviceAccounts"),
        }
    }
}
