#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkteamMemberDefinitionCognitoMemberDefinition {
    /// An identifier for an application client. You must create the app client ID using Amazon Cognito.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// An identifier for a user group.
    #[builder(into)]
    #[serde(rename = "userGroup")]
    pub r#user_group: String,
    /// An identifier for a user pool. The user pool must be in the same region as the service that you are calling.
    #[builder(into)]
    #[serde(rename = "userPool")]
    pub r#user_pool: String,
}
