#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolLambdaConfig {
    #[builder(into)]
    pub r#create_auth_challenge: String,
    #[builder(into)]
    pub r#custom_email_senders: Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomEmailSender>,
    #[builder(into)]
    pub r#custom_message: String,
    #[builder(into)]
    pub r#custom_sms_senders: Vec<super::super::types::cognito::GetUserPoolLambdaConfigCustomSmsSender>,
    #[builder(into)]
    pub r#define_auth_challenge: String,
    #[builder(into)]
    pub r#kms_key_id: String,
    #[builder(into)]
    pub r#post_authentication: String,
    #[builder(into)]
    pub r#post_confirmation: String,
    #[builder(into)]
    pub r#pre_authentication: String,
    #[builder(into)]
    pub r#pre_sign_up: String,
    #[builder(into)]
    pub r#pre_token_generation: String,
    #[builder(into)]
    pub r#pre_token_generation_configs: Vec<super::super::types::cognito::GetUserPoolLambdaConfigPreTokenGenerationConfig>,
    #[builder(into)]
    pub r#user_migration: String,
    #[builder(into)]
    pub r#verify_auth_challenge_response: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolLambdaConfig {
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
                    "customEmailSenders",
                    &self.r#custom_email_senders,
                ),
                to_pulumi_object_field(
                    "customMessage",
                    &self.r#custom_message,
                ),
                to_pulumi_object_field(
                    "customSmsSenders",
                    &self.r#custom_sms_senders,
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
                    "preTokenGenerationConfigs",
                    &self.r#pre_token_generation_configs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolLambdaConfig {
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
                    r#custom_email_senders: {
                        let field_value = match fields_map.get("customEmailSenders") {
                            Some(value) => value,
                            None => bail!("Missing field 'customEmailSenders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#custom_sms_senders: {
                        let field_value = match fields_map.get("customSmsSenders") {
                            Some(value) => value,
                            None => bail!("Missing field 'customSmsSenders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#pre_token_generation_configs: {
                        let field_value = match fields_map.get("preTokenGenerationConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'preTokenGenerationConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
