#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolLambdaConfig {
    /// ARN of the lambda creating an authentication challenge.
    #[builder(into)]
    #[serde(rename = "createAuthChallenge")]
    pub r#create_auth_challenge: Option<String>,
    /// A custom email sender AWS Lambda trigger. See custom_email_sender Below.
    #[builder(into)]
    #[serde(rename = "customEmailSender")]
    pub r#custom_email_sender: Option<Box<super::super::types::cognito::UserPoolLambdaConfigCustomEmailSender>>,
    /// Custom Message AWS Lambda trigger.
    #[builder(into)]
    #[serde(rename = "customMessage")]
    pub r#custom_message: Option<String>,
    /// A custom SMS sender AWS Lambda trigger. See custom_sms_sender Below.
    #[builder(into)]
    #[serde(rename = "customSmsSender")]
    pub r#custom_sms_sender: Option<Box<super::super::types::cognito::UserPoolLambdaConfigCustomSmsSender>>,
    /// Defines the authentication challenge.
    #[builder(into)]
    #[serde(rename = "defineAuthChallenge")]
    pub r#define_auth_challenge: Option<String>,
    /// The Amazon Resource Name of Key Management Service Customer master keys. Amazon Cognito uses the key to encrypt codes and temporary passwords sent to CustomEmailSender and CustomSMSSender.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Post-authentication AWS Lambda trigger.
    #[builder(into)]
    #[serde(rename = "postAuthentication")]
    pub r#post_authentication: Option<String>,
    /// Post-confirmation AWS Lambda trigger.
    #[builder(into)]
    #[serde(rename = "postConfirmation")]
    pub r#post_confirmation: Option<String>,
    /// Pre-authentication AWS Lambda trigger.
    #[builder(into)]
    #[serde(rename = "preAuthentication")]
    pub r#pre_authentication: Option<String>,
    /// Pre-registration AWS Lambda trigger.
    #[builder(into)]
    #[serde(rename = "preSignUp")]
    pub r#pre_sign_up: Option<String>,
    /// Allow to customize identity token claims before token generation. Set this parameter for legacy purposes; for new instances of pre token generation triggers, set the lambda_arn of `pre_token_generation_config`.
    #[builder(into)]
    #[serde(rename = "preTokenGeneration")]
    pub r#pre_token_generation: Option<String>,
    /// Allow to customize access tokens. See pre_token_configuration_type
    #[builder(into)]
    #[serde(rename = "preTokenGenerationConfig")]
    pub r#pre_token_generation_config: Option<Box<super::super::types::cognito::UserPoolLambdaConfigPreTokenGenerationConfig>>,
    /// User migration Lambda config type.
    #[builder(into)]
    #[serde(rename = "userMigration")]
    pub r#user_migration: Option<String>,
    /// Verifies the authentication challenge response.
    #[builder(into)]
    #[serde(rename = "verifyAuthChallengeResponse")]
    pub r#verify_auth_challenge_response: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserPoolLambdaConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("create_auth_challenge".to_string(), self.r#create_auth_challenge.to_pulumi_value().await);
            map.insert("custom_email_sender".to_string(), self.r#custom_email_sender.to_pulumi_value().await);
            map.insert("custom_message".to_string(), self.r#custom_message.to_pulumi_value().await);
            map.insert("custom_sms_sender".to_string(), self.r#custom_sms_sender.to_pulumi_value().await);
            map.insert("define_auth_challenge".to_string(), self.r#define_auth_challenge.to_pulumi_value().await);
            map.insert("kms_key_id".to_string(), self.r#kms_key_id.to_pulumi_value().await);
            map.insert("post_authentication".to_string(), self.r#post_authentication.to_pulumi_value().await);
            map.insert("post_confirmation".to_string(), self.r#post_confirmation.to_pulumi_value().await);
            map.insert("pre_authentication".to_string(), self.r#pre_authentication.to_pulumi_value().await);
            map.insert("pre_sign_up".to_string(), self.r#pre_sign_up.to_pulumi_value().await);
            map.insert("pre_token_generation".to_string(), self.r#pre_token_generation.to_pulumi_value().await);
            map.insert("pre_token_generation_config".to_string(), self.r#pre_token_generation_config.to_pulumi_value().await);
            map.insert("user_migration".to_string(), self.r#user_migration.to_pulumi_value().await);
            map.insert("verify_auth_challenge_response".to_string(), self.r#verify_auth_challenge_response.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolLambdaConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#create_auth_challenge: {
                        let field_value = match fields_map.get("create_auth_challenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_auth_challenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_email_sender: {
                        let field_value = match fields_map.get("custom_email_sender") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_email_sender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cognito::UserPoolLambdaConfigCustomEmailSender>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_message: {
                        let field_value = match fields_map.get("custom_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_sms_sender: {
                        let field_value = match fields_map.get("custom_sms_sender") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_sms_sender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cognito::UserPoolLambdaConfigCustomSmsSender>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#define_auth_challenge: {
                        let field_value = match fields_map.get("define_auth_challenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'define_auth_challenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_authentication: {
                        let field_value = match fields_map.get("post_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_confirmation: {
                        let field_value = match fields_map.get("post_confirmation") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_confirmation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_authentication: {
                        let field_value = match fields_map.get("pre_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_sign_up: {
                        let field_value = match fields_map.get("pre_sign_up") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_sign_up' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation: {
                        let field_value = match fields_map.get("pre_token_generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_token_generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation_config: {
                        let field_value = match fields_map.get("pre_token_generation_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_token_generation_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cognito::UserPoolLambdaConfigPreTokenGenerationConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_migration: {
                        let field_value = match fields_map.get("user_migration") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_migration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#verify_auth_challenge_response: {
                        let field_value = match fields_map.get("verify_auth_challenge_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_auth_challenge_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
