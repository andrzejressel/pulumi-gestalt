#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOrganizationalUnitChildAccountsAccount {
    /// The Amazon Resource Name (ARN) of the account.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The email address associated with the AWS account.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// Parent identifier of the organizational units.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The friendly name of the account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The status of the account in the organization.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
