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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "createAuthChallenge",
                    &self.r#create_auth_challenge,
                ),
                to_pulumi_object_field(
                    "customEmailSender",
                    &self.r#custom_email_sender,
                ),
                to_pulumi_object_field(
                    "customMessage",
                    &self.r#custom_message,
                ),
                to_pulumi_object_field(
                    "customSmsSender",
                    &self.r#custom_sms_sender,
                ),
                to_pulumi_object_field(
                    "defineAuthChallenge",
                    &self.r#define_auth_challenge,
                ),
                to_pulumi_object_field(
                    "kmsKeyId",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "postAuthentication",
                    &self.r#post_authentication,
                ),
                to_pulumi_object_field(
                    "postConfirmation",
                    &self.r#post_confirmation,
                ),
                to_pulumi_object_field(
                    "preAuthentication",
                    &self.r#pre_authentication,
                ),
                to_pulumi_object_field(
                    "preSignUp",
                    &self.r#pre_sign_up,
                ),
                to_pulumi_object_field(
                    "preTokenGeneration",
                    &self.r#pre_token_generation,
                ),
                to_pulumi_object_field(
                    "preTokenGenerationConfig",
                    &self.r#pre_token_generation_config,
                ),
                to_pulumi_object_field(
                    "userMigration",
                    &self.r#user_migration,
                ),
                to_pulumi_object_field(
                    "verifyAuthChallengeResponse",
                    &self.r#verify_auth_challenge_response,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserPoolLambdaConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#create_auth_challenge: {
                        let field_value = match fields_map.get("createAuthChallenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'createAuthChallenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_email_sender: {
                        let field_value = match fields_map.get("customEmailSender") {
                            Some(value) => value,
                            None => bail!("Missing field 'customEmailSender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_message: {
                        let field_value = match fields_map.get("customMessage") {
                            Some(value) => value,
                            None => bail!("Missing field 'customMessage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_sms_sender: {
                        let field_value = match fields_map.get("customSmsSender") {
                            Some(value) => value,
                            None => bail!("Missing field 'customSmsSender' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#define_auth_challenge: {
                        let field_value = match fields_map.get("defineAuthChallenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'defineAuthChallenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kmsKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'kmsKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_authentication: {
                        let field_value = match fields_map.get("postAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'postAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_confirmation: {
                        let field_value = match fields_map.get("postConfirmation") {
                            Some(value) => value,
                            None => bail!("Missing field 'postConfirmation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_authentication: {
                        let field_value = match fields_map.get("preAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'preAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_sign_up: {
                        let field_value = match fields_map.get("preSignUp") {
                            Some(value) => value,
                            None => bail!("Missing field 'preSignUp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation: {
                        let field_value = match fields_map.get("preTokenGeneration") {
                            Some(value) => value,
                            None => bail!("Missing field 'preTokenGeneration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation_config: {
                        let field_value = match fields_map.get("preTokenGenerationConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'preTokenGenerationConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_migration: {
                        let field_value = match fields_map.get("userMigration") {
                            Some(value) => value,
                            None => bail!("Missing field 'userMigration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify_auth_challenge_response: {
                        let field_value = match fields_map.get("verifyAuthChallengeResponse") {
                            Some(value) => value,
                            None => bail!("Missing field 'verifyAuthChallengeResponse' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
