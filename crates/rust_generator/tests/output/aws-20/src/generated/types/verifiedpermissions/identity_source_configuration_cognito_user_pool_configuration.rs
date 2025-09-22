#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentitySourceConfigurationCognitoUserPoolConfiguration {
    /// The unique application client IDs that are associated with the specified Amazon Cognito user pool.
    #[builder(into)]
    #[serde(rename = "clientIds")]
    pub r#client_ids: Option<Vec<String>>,
    /// The type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
    #[builder(into)]
    #[serde(rename = "groupConfiguration")]
    pub r#group_configuration: Box<Option<super::super::types::verifiedpermissions::IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration>>,
    /// The Amazon Resource Name (ARN) of the Amazon Cognito user pool that contains the identities to be authorized.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: String,
}
