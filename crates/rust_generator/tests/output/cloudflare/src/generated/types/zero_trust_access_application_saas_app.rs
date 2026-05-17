#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessApplicationSaasApp {
    /// The lifetime of the Access Token after creation. Valid units are `m` and `h`. Must be greater than or equal to 1m and less than or equal to 24h.
    #[builder(into)]
    #[serde(rename = "accessTokenLifetime")]
    pub r#access_token_lifetime: Option<String>,
    /// Allow PKCE flow without a client secret.
    #[builder(into)]
    #[serde(rename = "allowPkceWithoutClientSecret")]
    pub r#allow_pkce_without_client_secret: Option<bool>,
    /// The URL where this applications tile redirects users.
    #[builder(into)]
    #[serde(rename = "appLauncherUrl")]
    pub r#app_launcher_url: Option<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: Option<String>,
    /// The application client id.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The application client secret, only returned on initial apply.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// The service provider's endpoint that is responsible for receiving and parsing a SAML assertion.
    #[builder(into)]
    #[serde(rename = "consumerServiceUrl")]
    pub r#consumer_service_url: Option<String>,
    /// Custom attribute mapped from IDPs.
    #[builder(into)]
    #[serde(rename = "customAttributes")]
    pub r#custom_attributes: Option<Vec<super::types::ZeroTrustAccessApplicationSaasAppCustomAttribute>>,
    /// Custom claim mapped from IDPs.
    #[builder(into)]
    #[serde(rename = "customClaims")]
    pub r#custom_claims: Option<Vec<super::types::ZeroTrustAccessApplicationSaasAppCustomClaim>>,
    /// The relay state used if not provided by the identity provider.
    #[builder(into)]
    #[serde(rename = "defaultRelayState")]
    pub r#default_relay_state: Option<String>,
    /// The OIDC flows supported by this application.
    #[builder(into)]
    #[serde(rename = "grantTypes")]
    pub r#grant_types: Option<Vec<String>>,
    /// A regex to filter Cloudflare groups returned in ID token and userinfo endpoint.
    #[builder(into)]
    #[serde(rename = "groupFilterRegex")]
    pub r#group_filter_regex: Option<String>,
    /// Hybrid and Implicit Flow options.
    #[builder(into)]
    #[serde(rename = "hybridAndImplicitOptions")]
    pub r#hybrid_and_implicit_options: Option<Box<super::types::ZeroTrustAccessApplicationSaasAppHybridAndImplicitOptions>>,
    /// The unique identifier for the SaaS application.
    #[builder(into)]
    #[serde(rename = "idpEntityId")]
    pub r#idp_entity_id: Option<String>,
    /// The format of the name identifier sent to the SaaS application.
    #[builder(into)]
    #[serde(rename = "nameIdFormat")]
    pub r#name_id_format: Option<String>,
    /// A [JSONata](https://jsonata.org/) expression that transforms an application's user identities into a NameID value for its SAML assertion. This expression should evaluate to a singular string. The output of this expression can override the `name_id_format` setting.
    #[builder(into)]
    #[serde(rename = "nameIdTransformJsonata")]
    pub r#name_id_transform_jsonata: Option<String>,
    /// The public certificate that will be used to verify identities.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    /// The permitted URL's for Cloudflare to return Authorization codes and Access/ID tokens.
    #[builder(into)]
    #[serde(rename = "redirectUris")]
    pub r#redirect_uris: Option<Vec<String>>,
    /// Refresh token grant options.
    #[builder(into)]
    #[serde(rename = "refreshTokenOptions")]
    pub r#refresh_token_options: Option<Vec<super::types::ZeroTrustAccessApplicationSaasAppRefreshTokenOption>>,
    /// A [JSONata](https://jsonata.org/) expression that transforms an application's user identities into attribute assertions in the SAML response. The expression can transform id, email, name, and groups values. It can also transform fields listed in the saml*attributes or oidc*fields of the identity provider used to authenticate. The output of this expression must be a JSON object.
    #[builder(into)]
    #[serde(rename = "samlAttributeTransformJsonata")]
    pub r#saml_attribute_transform_jsonata: Option<String>,
    /// Define the user information shared with access.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    /// A globally unique name for an identity or service provider.
    #[builder(into)]
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Option<String>,
    /// The endpoint where the SaaS application will send login requests.
    #[builder(into)]
    #[serde(rename = "ssoEndpoint")]
    pub r#sso_endpoint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustAccessApplicationSaasApp {
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
                "access_token_lifetime".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_token_lifetime,
                )
                .await,
            );
            map.insert(
                "allow_pkce_without_client_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_pkce_without_client_secret,
                )
                .await,
            );
            map.insert(
                "app_launcher_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_launcher_url,
                )
                .await,
            );
            map.insert(
                "auth_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_type,
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
                "consumer_service_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumer_service_url,
                )
                .await,
            );
            map.insert(
                "custom_attributes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_attributes,
                )
                .await,
            );
            map.insert(
                "custom_claims".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_claims,
                )
                .await,
            );
            map.insert(
                "default_relay_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_relay_state,
                )
                .await,
            );
            map.insert(
                "grant_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grant_types,
                )
                .await,
            );
            map.insert(
                "group_filter_regex".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_filter_regex,
                )
                .await,
            );
            map.insert(
                "hybrid_and_implicit_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hybrid_and_implicit_options,
                )
                .await,
            );
            map.insert(
                "idp_entity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idp_entity_id,
                )
                .await,
            );
            map.insert(
                "name_id_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_id_format,
                )
                .await,
            );
            map.insert(
                "name_id_transform_jsonata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_id_transform_jsonata,
                )
                .await,
            );
            map.insert(
                "public_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_key,
                )
                .await,
            );
            map.insert(
                "redirect_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_uris,
                )
                .await,
            );
            map.insert(
                "refresh_token_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refresh_token_options,
                )
                .await,
            );
            map.insert(
                "saml_attribute_transform_jsonata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#saml_attribute_transform_jsonata,
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
                "sp_entity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sp_entity_id,
                )
                .await,
            );
            map.insert(
                "sso_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sso_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustAccessApplicationSaasApp {
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
                    r#access_token_lifetime: {
                        let field_value = match fields_map.get("access_token_lifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_token_lifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_pkce_without_client_secret: {
                        let field_value = match fields_map.get("allow_pkce_without_client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_pkce_without_client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_launcher_url: {
                        let field_value = match fields_map.get("app_launcher_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_launcher_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#consumer_service_url: {
                        let field_value = match fields_map.get("consumer_service_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_service_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_attributes: {
                        let field_value = match fields_map.get("custom_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_claims: {
                        let field_value = match fields_map.get("custom_claims") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_claims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_relay_state: {
                        let field_value = match fields_map.get("default_relay_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_relay_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grant_types: {
                        let field_value = match fields_map.get("grant_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'grant_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_filter_regex: {
                        let field_value = match fields_map.get("group_filter_regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_filter_regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hybrid_and_implicit_options: {
                        let field_value = match fields_map.get("hybrid_and_implicit_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'hybrid_and_implicit_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_entity_id: {
                        let field_value = match fields_map.get("idp_entity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp_entity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_id_format: {
                        let field_value = match fields_map.get("name_id_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_id_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_id_transform_jsonata: {
                        let field_value = match fields_map.get("name_id_transform_jsonata") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_id_transform_jsonata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_key: {
                        let field_value = match fields_map.get("public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_uris: {
                        let field_value = match fields_map.get("redirect_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_token_options: {
                        let field_value = match fields_map.get("refresh_token_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_token_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#saml_attribute_transform_jsonata: {
                        let field_value = match fields_map.get("saml_attribute_transform_jsonata") {
                            Some(value) => value,
                            None => bail!("Missing field 'saml_attribute_transform_jsonata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sp_entity_id: {
                        let field_value = match fields_map.get("sp_entity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'sp_entity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_endpoint: {
                        let field_value = match fields_map.get("sso_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'sso_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
