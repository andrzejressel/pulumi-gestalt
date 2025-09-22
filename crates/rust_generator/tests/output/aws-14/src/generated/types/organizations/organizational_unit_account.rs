#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrganizationalUnitAccount {
    /// ARN of the organizational unit
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Option<String>,
    /// Email of the account
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Identifier of the organization unit
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name for the organizational unit
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
