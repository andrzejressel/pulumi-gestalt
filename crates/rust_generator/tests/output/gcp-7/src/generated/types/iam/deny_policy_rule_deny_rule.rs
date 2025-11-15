#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DenyPolicyRuleDenyRule {
    /// User defined CEVAL expression. A CEVAL expression is used to specify match criteria such as origin.ip, source.region_code and contents in the request header.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "denialCondition")]
    pub r#denial_condition: Option<Box<super::super::types::iam::DenyPolicyRuleDenyRuleDenialCondition>>,
    /// The permissions that are explicitly denied by this rule. Each permission uses the format `{service-fqdn}/{resource}.{verb}`,
    /// where `{service-fqdn}` is the fully qualified domain name for the service. For example, `iam.googleapis.com/roles.list`.
    #[builder(into)]
    #[serde(rename = "deniedPermissions")]
    pub r#denied_permissions: Option<Vec<String>>,
    /// The identities that are prevented from using one or more permissions on Google Cloud resources.
    #[builder(into)]
    #[serde(rename = "deniedPrincipals")]
    pub r#denied_principals: Option<Vec<String>>,
    /// Specifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.
    /// If a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.
    /// The excluded permissions can be specified using the same syntax as deniedPermissions.
    #[builder(into)]
    #[serde(rename = "exceptionPermissions")]
    pub r#exception_permissions: Option<Vec<String>>,
    /// The identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.
    /// For example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group.
    #[builder(into)]
    #[serde(rename = "exceptionPrincipals")]
    pub r#exception_principals: Option<Vec<String>>,
}
