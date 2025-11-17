#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDelegatedAdministratorsDelegatedAdministrator {
    /// The ARN of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The date when the account was made a delegated administrator.
    #[builder(into)]
    #[serde(rename = "delegationEnabledDate")]
    pub r#delegation_enabled_date: String,
    /// The email address that is associated with the delegated administrator's AWS account.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// The unique identifier (ID) of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The method by which the delegated administrator's account joined the organization.
    #[builder(into)]
    #[serde(rename = "joinedMethod")]
    pub r#joined_method: String,
    /// The date when the delegated administrator's account became a part of the organization.
    #[builder(into)]
    #[serde(rename = "joinedTimestamp")]
    pub r#joined_timestamp: String,
    /// The friendly name of the delegated administrator's account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The status of the delegated administrator's account in the organization.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
