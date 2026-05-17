#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigDecryptedCredential {
    /// Auth token credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authToken")]
    pub r#auth_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialAuthToken>>,
    /// Credential type associated with auth configs.
    #[builder(into)]
    #[serde(rename = "credentialType")]
    pub r#credential_type: String,
    /// JWT credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "jwt")]
    pub r#jwt: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialJwt>>,
    /// OAuth2 authorization code credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2AuthorizationCode")]
    pub r#oauth_2_authorization_code: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2AuthorizationCode>>,
    /// OAuth2 client credentials.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oauth2ClientCredentials")]
    pub r#oauth_2_client_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentials>>,
    /// Google OIDC ID Token.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOidcToken>>,
    /// Service account credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAccountCredentials")]
    pub r#service_account_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialServiceAccountCredentials>>,
    /// Username and password credential.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "usernameAndPassword")]
    pub r#username_and_password: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialUsernameAndPassword>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthConfigDecryptedCredential {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "auth_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_token,
                )
                .await,
            );
            map.insert(
                "credential_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#credential_type,
                )
                .await,
            );
            map.insert(
                "jwt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt,
                )
                .await,
            );
            map.insert(
                "oauth_2_authorization_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2_authorization_code,
                )
                .await,
            );
            map.insert(
                "oauth_2_client_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oauth_2_client_credentials,
                )
                .await,
            );
            map.insert(
                "oidc_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oidc_token,
                )
                .await,
            );
            map.insert(
                "service_account_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_credentials,
                )
                .await,
            );
            map.insert(
                "username_and_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username_and_password,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthConfigDecryptedCredential {
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
                    r#auth_token: {
                        let field_value = match fields_map.get("auth_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credential_type: {
                        let field_value = match fields_map.get("credential_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'credential_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt: {
                        let field_value = match fields_map.get("jwt") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_authorization_code: {
                        let field_value = match fields_map.get("oauth_2_authorization_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth_2_authorization_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#oidc_token: {
                        let field_value = match fields_map.get("oidc_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'oidc_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_credentials: {
                        let field_value = match fields_map.get("service_account_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username_and_password: {
                        let field_value = match fields_map.get("username_and_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'username_and_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
