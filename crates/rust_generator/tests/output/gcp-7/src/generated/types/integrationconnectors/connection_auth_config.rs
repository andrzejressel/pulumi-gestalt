#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionAuthConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Option<Vec<super::super::types::integrationconnectors::ConnectionAuthConfigAdditionalVariable>>,
    /// The type of authentication configured.
    #[builder(into)]
    #[serde(rename = "authKey")]
    pub r#auth_key: Option<String>,
    /// authType of the Connection
    /// Possible values are: `USER_PASSWORD`.
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: String,
    /// Parameters to support Oauth 2.0 Auth Code Grant Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2AuthCodeFlow")]
    pub r#oauth_2_auth_code_flow: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2AuthCodeFlow>>,
    /// OAuth3 Client Credentials for Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2ClientCredentials")]
    pub r#oauth_2_client_credentials: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2ClientCredentials>>,
    /// OAuth2 JWT Bearer for Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2JwtBearer")]
    pub r#oauth_2_jwt_bearer: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearer>>,
    /// SSH Public Key for Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sshPublicKey")]
    pub r#ssh_public_key: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKey>>,
    /// User password for Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userPassword")]
    pub r#user_password: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigUserPassword>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionAuthConfig {
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
                    "additional_variables",
                    &self.r#additional_variables,
                ),
                to_pulumi_object_field(
                    "auth_key",
                    &self.r#auth_key,
                ),
                to_pulumi_object_field(
                    "auth_type",
                    &self.r#auth_type,
                ),
                to_pulumi_object_field(
                    "oauth_2_auth_code_flow",
                    &self.r#oauth_2_auth_code_flow,
                ),
                to_pulumi_object_field(
                    "oauth_2_client_credentials",
                    &self.r#oauth_2_client_credentials,
                ),
                to_pulumi_object_field(
                    "oauth_2_jwt_bearer",
                    &self.r#oauth_2_jwt_bearer,
                ),
                to_pulumi_object_field(
                    "ssh_public_key",
                    &self.r#ssh_public_key,
                ),
                to_pulumi_object_field(
                    "user_password",
                    &self.r#user_password,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionAuthConfig {
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
                    r#additional_variables: {
                        let field_value = match fields_map.get("additional_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_key: {
                        let field_value = match fields_map.get("auth_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_type: {
                        let field_value = match fields_map.get("auth_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_auth_code_flow: {
                        let field_value = match fields_map.get("oauth_2_auth_code_flow") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_auth_code_flow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_credentials: {
                        let field_value = match fields_map.get("oauth_2_client_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_client_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_jwt_bearer: {
                        let field_value = match fields_map.get("oauth_2_jwt_bearer") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_jwt_bearer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_public_key: {
                        let field_value = match fields_map.get("ssh_public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_password: {
                        let field_value = match fields_map.get("user_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
