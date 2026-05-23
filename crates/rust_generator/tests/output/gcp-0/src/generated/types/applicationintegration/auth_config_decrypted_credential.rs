#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigDecryptedCredential {
    /// Auth token credential.
    /// Structure is documented below.
    #[builder(into)]
    pub r#auth_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialAuthToken>>,
    /// Credential type associated with auth configs.
    #[builder(into)]
    pub r#credential_type: String,
    /// JWT credential.
    /// Structure is documented below.
    #[builder(into)]
    pub r#jwt: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialJwt>>,
    /// OAuth2 authorization code credential.
    /// Structure is documented below.
    #[builder(into)]
    pub r#oauth_2_authorization_code: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2AuthorizationCode>>,
    /// OAuth2 client credentials.
    /// Structure is documented below.
    #[builder(into)]
    pub r#oauth_2_client_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentials>>,
    /// Google OIDC ID Token.
    /// Structure is documented below.
    #[builder(into)]
    pub r#oidc_token: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOidcToken>>,
    /// Service account credential.
    /// Structure is documented below.
    #[builder(into)]
    pub r#service_account_credentials: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialServiceAccountCredentials>>,
    /// Username and password credential.
    /// Structure is documented below.
    #[builder(into)]
    pub r#username_and_password: Option<Box<super::super::types::applicationintegration::AuthConfigDecryptedCredentialUsernameAndPassword>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthConfigDecryptedCredential {
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
                    "authToken",
                    &self.r#auth_token,
                ),
                to_pulumi_object_field(
                    "credentialType",
                    &self.r#credential_type,
                ),
                to_pulumi_object_field(
                    "jwt",
                    &self.r#jwt,
                ),
                to_pulumi_object_field(
                    "oauth2AuthorizationCode",
                    &self.r#oauth_2_authorization_code,
                ),
                to_pulumi_object_field(
                    "oauth2ClientCredentials",
                    &self.r#oauth_2_client_credentials,
                ),
                to_pulumi_object_field(
                    "oidcToken",
                    &self.r#oidc_token,
                ),
                to_pulumi_object_field(
                    "serviceAccountCredentials",
                    &self.r#service_account_credentials,
                ),
                to_pulumi_object_field(
                    "usernameAndPassword",
                    &self.r#username_and_password,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("authToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'authToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credential_type: {
                        let field_value = match fields_map.get("credentialType") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentialType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("oauth2AuthorizationCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth2AuthorizationCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oauth_2_client_credentials: {
                        let field_value = match fields_map.get("oauth2ClientCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'oauth2ClientCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oidc_token: {
                        let field_value = match fields_map.get("oidcToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'oidcToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_credentials: {
                        let field_value = match fields_map.get("serviceAccountCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username_and_password: {
                        let field_value = match fields_map.get("usernameAndPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'usernameAndPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
