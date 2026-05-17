#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessIdentityProviderConfig {
    #[builder(into)]
    #[serde(rename = "apiToken")]
    pub r#api_token: Option<String>,
    #[builder(into)]
    #[serde(rename = "appsDomain")]
    pub r#apps_domain: Option<String>,
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "authorizationServerId")]
    pub r#authorization_server_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAccount")]
    pub r#centrify_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "centrifyAppId")]
    pub r#centrify_app_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "certsUrl")]
    pub r#certs_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "claims")]
    pub r#claims: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    #[builder(into)]
    #[serde(rename = "conditionalAccessEnabled")]
    pub r#conditional_access_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "directoryId")]
    pub r#directory_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailAttributeName")]
    pub r#email_attribute_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "emailClaimName")]
    pub r#email_claim_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "idpPublicCert")]
    pub r#idp_public_cert: Option<String>,
    #[builder(into)]
    #[serde(rename = "issuerUrl")]
    pub r#issuer_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "oktaAccount")]
    pub r#okta_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "oneloginAccount")]
    pub r#onelogin_account: Option<String>,
    #[builder(into)]
    #[serde(rename = "pingEnvId")]
    pub r#ping_env_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "pkceEnabled")]
    pub r#pkce_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "redirectUrl")]
    pub r#redirect_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "signRequest")]
    pub r#sign_request: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ssoTargetUrl")]
    pub r#sso_target_url: Option<String>,
    #[builder(into)]
    #[serde(rename = "supportGroups")]
    pub r#support_groups: Option<bool>,
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessIdentityProviderConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "api_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_token,
                )
                .await,
            );
            map.insert(
                "apps_domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apps_domain,
                )
                .await,
            );
            map.insert(
                "attributes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attributes,
                )
                .await,
            );
            map.insert(
                "auth_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_url,
                )
                .await,
            );
            map.insert(
                "authorization_server_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_server_id,
                )
                .await,
            );
            map.insert(
                "centrify_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#centrify_account,
                )
                .await,
            );
            map.insert(
                "centrify_app_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#centrify_app_id,
                )
                .await,
            );
            map.insert(
                "certs_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certs_url,
                )
                .await,
            );
            map.insert(
                "claims".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#claims,
                )
                .await,
            );
            map.insert(
                "client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_id,
                )
                .await,
            );
            map.insert(
                "client_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_secret,
                )
                .await,
            );
            map.insert(
                "conditional_access_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conditional_access_enabled,
                )
                .await,
            );
            map.insert(
                "directory_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#directory_id,
                )
                .await,
            );
            map.insert(
                "email_attribute_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_attribute_name,
                )
                .await,
            );
            map.insert(
                "email_claim_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_claim_name,
                )
                .await,
            );
            map.insert(
                "idp_public_cert".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idp_public_cert,
                )
                .await,
            );
            map.insert(
                "issuer_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer_url,
                )
                .await,
            );
            map.insert(
                "okta_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#okta_account,
                )
                .await,
            );
            map.insert(
                "onelogin_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#onelogin_account,
                )
                .await,
            );
            map.insert(
                "ping_env_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ping_env_id,
                )
                .await,
            );
            map.insert(
                "pkce_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pkce_enabled,
                )
                .await,
            );
            map.insert(
                "redirect_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_url,
                )
                .await,
            );
            map.insert(
                "scopes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scopes,
                )
                .await,
            );
            map.insert(
                "sign_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sign_request,
                )
                .await,
            );
            map.insert(
                "sso_target_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sso_target_url,
                )
                .await,
            );
            map.insert(
                "support_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#support_groups,
                )
                .await,
            );
            map.insert(
                "token_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_url,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessIdentityProviderConfig {
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
                    r#api_token: {
                        let field_value = match fields_map.get("api_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apps_domain: {
                        let field_value = match fields_map.get("apps_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'apps_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_url: {
                        let field_value = match fields_map.get("auth_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorization_server_id: {
                        let field_value = match fields_map.get("authorization_server_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_server_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#centrify_account: {
                        let field_value = match fields_map.get("centrify_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'centrify_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#centrify_app_id: {
                        let field_value = match fields_map.get("centrify_app_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'centrify_app_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certs_url: {
                        let field_value = match fields_map.get("certs_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'certs_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#claims: {
                        let field_value = match fields_map.get("claims") {
                            Some(value) => value,
                            None => bail!("Missing field 'claims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conditional_access_enabled: {
                        let field_value = match fields_map.get("conditional_access_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditional_access_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directory_id: {
                        let field_value = match fields_map.get("directory_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'directory_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_attribute_name: {
                        let field_value = match fields_map.get("email_attribute_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_attribute_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_claim_name: {
                        let field_value = match fields_map.get("email_claim_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_claim_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_public_cert: {
                        let field_value = match fields_map.get("idp_public_cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp_public_cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_url: {
                        let field_value = match fields_map.get("issuer_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#okta_account: {
                        let field_value = match fields_map.get("okta_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'okta_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#onelogin_account: {
                        let field_value = match fields_map.get("onelogin_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'onelogin_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ping_env_id: {
                        let field_value = match fields_map.get("ping_env_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ping_env_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pkce_enabled: {
                        let field_value = match fields_map.get("pkce_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'pkce_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_url: {
                        let field_value = match fields_map.get("redirect_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scopes: {
                        let field_value = match fields_map.get("scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sign_request: {
                        let field_value = match fields_map.get("sign_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'sign_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_target_url: {
                        let field_value = match fields_map.get("sso_target_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'sso_target_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#support_groups: {
                        let field_value = match fields_map.get("support_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'support_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_url: {
                        let field_value = match fields_map.get("token_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
