#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountNetworkProfileAccountAccess {
    /// Specifies the default action for the account access. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Option<String>,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<super::super::types::batch::AccountNetworkProfileAccountAccessIpRule>>,
}
