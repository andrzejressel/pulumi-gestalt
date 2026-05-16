#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolLambdaConfig {
    #[builder(into)]
    #[serde(rename = "createAuthChallenge")]
    pub r#create_auth_challenge: String,
    #[builder(into)]
    #[serde(rename = "customEmailSenders")]
    pub r#custom_email_senders: Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomEmailSender>,
    #[builder(into)]
    #[serde(rename = "customMessage")]
    pub r#custom_message: String,
    #[builder(into)]
    #[serde(rename = "customSmsSenders")]
    pub r#custom_sms_senders: Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomSmsSender>,
    #[builder(into)]
    #[serde(rename = "defineAuthChallenge")]
    pub r#define_auth_challenge: String,
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: String,
    #[builder(into)]
    #[serde(rename = "postAuthentication")]
    pub r#post_authentication: String,
    #[builder(into)]
    #[serde(rename = "postConfirmation")]
    pub r#post_confirmation: String,
    #[builder(into)]
    #[serde(rename = "preAuthentication")]
    pub r#pre_authentication: String,
    #[builder(into)]
    #[serde(rename = "preSignUp")]
    pub r#pre_sign_up: String,
    #[builder(into)]
    #[serde(rename = "preTokenGeneration")]
    pub r#pre_token_generation: String,
    #[builder(into)]
    #[serde(rename = "preTokenGenerationConfigs")]
    pub r#pre_token_generation_configs: Vec<super::super::types::cognito::GetUserPoolLambdaConfigPreTokenGenerationConfig>,
    #[builder(into)]
    #[serde(rename = "userMigration")]
    pub r#user_migration: String,
    #[builder(into)]
    #[serde(rename = "verifyAuthChallengeResponse")]
    pub r#verify_auth_challenge_response: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolLambdaConfig {
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
            map.insert("custom_email_senders".to_string(), self.r#custom_email_senders.to_pulumi_value().await);
            map.insert("custom_message".to_string(), self.r#custom_message.to_pulumi_value().await);
            map.insert("custom_sms_senders".to_string(), self.r#custom_sms_senders.to_pulumi_value().await);
            map.insert("define_auth_challenge".to_string(), self.r#define_auth_challenge.to_pulumi_value().await);
            map.insert("kms_key_id".to_string(), self.r#kms_key_id.to_pulumi_value().await);
            map.insert("post_authentication".to_string(), self.r#post_authentication.to_pulumi_value().await);
            map.insert("post_confirmation".to_string(), self.r#post_confirmation.to_pulumi_value().await);
            map.insert("pre_authentication".to_string(), self.r#pre_authentication.to_pulumi_value().await);
            map.insert("pre_sign_up".to_string(), self.r#pre_sign_up.to_pulumi_value().await);
            map.insert("pre_token_generation".to_string(), self.r#pre_token_generation.to_pulumi_value().await);
            map.insert("pre_token_generation_configs".to_string(), self.r#pre_token_generation_configs.to_pulumi_value().await);
            map.insert("user_migration".to_string(), self.r#user_migration.to_pulumi_value().await);
            map.insert("verify_auth_challenge_response".to_string(), self.r#verify_auth_challenge_response.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolLambdaConfig {
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
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_email_senders: {
                        let field_value = match fields_map.get("custom_email_senders") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_email_senders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomEmailSender> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_message: {
                        let field_value = match fields_map.get("custom_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_sms_senders: {
                        let field_value = match fields_map.get("custom_sms_senders") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_sms_senders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomSmsSender> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#define_auth_challenge: {
                        let field_value = match fields_map.get("define_auth_challenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'define_auth_challenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_authentication: {
                        let field_value = match fields_map.get("post_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#post_confirmation: {
                        let field_value = match fields_map.get("post_confirmation") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_confirmation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_authentication: {
                        let field_value = match fields_map.get("pre_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_sign_up: {
                        let field_value = match fields_map.get("pre_sign_up") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_sign_up' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation: {
                        let field_value = match fields_map.get("pre_token_generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_token_generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pre_token_generation_configs: {
                        let field_value = match fields_map.get("pre_token_generation_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_token_generation_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cognito::GetUserPoolLambdaConfigPreTokenGenerationConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_migration: {
                        let field_value = match fields_map.get("user_migration") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_migration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#verify_auth_challenge_response: {
                        let field_value = match fields_map.get("verify_auth_challenge_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_auth_challenge_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
