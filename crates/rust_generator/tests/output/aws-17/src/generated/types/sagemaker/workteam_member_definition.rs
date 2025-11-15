#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkteamMemberDefinition {
    /// The Amazon Cognito user group that is part of the work team. See Cognito Member Definition details below.
    #[builder(into)]
    #[serde(rename = "cognitoMemberDefinition")]
    pub r#cognito_member_definition: Option<Box<super::super::types::sagemaker::WorkteamMemberDefinitionCognitoMemberDefinition>>,
    /// A list user groups that exist in your OIDC Identity Provider (IdP). One to ten groups can be used to create a single private work team. See Cognito Member Definition details below.
    #[builder(into)]
    #[serde(rename = "oidcMemberDefinition")]
    pub r#oidc_member_definition: Option<Box<super::super::types::sagemaker::WorkteamMemberDefinitionOidcMemberDefinition>>,
}
