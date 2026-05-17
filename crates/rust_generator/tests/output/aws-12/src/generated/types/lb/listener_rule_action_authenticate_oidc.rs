#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleActionAuthenticateOidc {
    /// The query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    /// The authorization endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: String,
    /// The OAuth 2.0 client identifier.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The OAuth 2.0 client secret.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: String,
    /// The OIDC issuer identifier of the IdP.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// The behavior if the user is not authenticated. Valid values: `deny`, `allow` and `authenticate`
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Option<String>,
    /// The set of user claims to be requested from the IdP.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
    /// The name of the cookie used to maintain session information.
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Option<String>,
    /// The maximum duration of the authentication session, in seconds.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Option<i32>,
    /// The token endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: String,
    /// The user info endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ListenerRuleActionAuthenticateOidc {
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
                "authentication_request_extra_params".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_request_extra_params,
                )
                .await,
            );
            map.insert(
                "authorization_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_endpoint,
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
                "issuer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer,
                )
                .await,
            );
            map.insert(
                "on_unauthenticated_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_unauthenticated_request,
                )
                .await,
            );
            map.insert(
                "scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scope,
                )
                .await,
            );
            map.insert(
                "session_cookie_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_cookie_name,
                )
                .await,
            );
            map.insert(
                "session_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_timeout,
                )
                .await,
            );
            map.insert(
                "token_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_endpoint,
                )
                .await,
            );
            map.insert(
                "user_info_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_info_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ListenerRuleActionAuthenticateOidc {
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
                    r#authentication_request_extra_params: {
                        let field_value = match fields_map.get("authentication_request_extra_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_request_extra_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorization_endpoint: {
                        let field_value = match fields_map.get("authorization_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_unauthenticated_request: {
                        let field_value = match fields_map.get("on_unauthenticated_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_unauthenticated_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_cookie_name: {
                        let field_value = match fields_map.get("session_cookie_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_cookie_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_timeout: {
                        let field_value = match fields_map.get("session_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_endpoint: {
                        let field_value = match fields_map.get("token_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_info_endpoint: {
                        let field_value = match fields_map.get("user_info_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_info_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
