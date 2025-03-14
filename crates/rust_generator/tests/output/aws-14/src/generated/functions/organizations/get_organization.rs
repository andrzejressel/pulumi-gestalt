#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_organization {
    #[allow(dead_code)]
    pub struct GetOrganizationResult {
        /// List of organization accounts including the master account. For a list excluding the master account, see the `non_master_accounts` attribute. All elements have these attributes:
        pub accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::organizations::GetOrganizationAccount>,
        >,
        /// ARN of the root
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A list of AWS service principal names that have integration enabled with your organization. Organization must have `feature_set` set to `ALL`. For additional information, see the [AWS Organizations User Guide](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html).
        pub aws_service_access_principals: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of Organizations policy types that are enabled in the Organization Root. Organization must have `feature_set` set to `ALL`. For additional information about valid policy types (e.g., `SERVICE_CONTROL_POLICY`), see the [AWS Organizations API Reference](https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnablePolicyType.html).
        pub enabled_policy_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// FeatureSet of the organization.
        pub feature_set: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the account that is designated as the master account for the organization.
        pub master_account_arn: pulumi_gestalt_rust::Output<String>,
        /// The email address that is associated with the AWS account that is designated as the master account for the organization.
        pub master_account_email: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier (ID) of the master account of an organization.
        pub master_account_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the master account of an organization.
        pub master_account_name: pulumi_gestalt_rust::Output<String>,
        /// List of organization accounts excluding the master account. For a list including the master account, see the `accounts` attribute. All elements have these attributes:
        pub non_master_accounts: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationNonMasterAccount,
            >,
        >,
        /// List of organization roots. All elements have these attributes:
        pub roots: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::organizations::GetOrganizationRoot>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetOrganizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getOrganization:getOrganization".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetOrganizationResult {
            accounts: o.get_field("accounts"),
            arn: o.get_field("arn"),
            aws_service_access_principals: o.get_field("awsServiceAccessPrincipals"),
            enabled_policy_types: o.get_field("enabledPolicyTypes"),
            feature_set: o.get_field("featureSet"),
            id: o.get_field("id"),
            master_account_arn: o.get_field("masterAccountArn"),
            master_account_email: o.get_field("masterAccountEmail"),
            master_account_id: o.get_field("masterAccountId"),
            master_account_name: o.get_field("masterAccountName"),
            non_master_accounts: o.get_field("nonMasterAccounts"),
            roots: o.get_field("roots"),
        }
    }
}
