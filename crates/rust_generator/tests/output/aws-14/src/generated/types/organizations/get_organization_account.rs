#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOrganizationAccount {
    /// ARN of the root
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Email of the account
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// Identifier of the root
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of the policy type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The status of the policy type as it relates to the associated root
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
