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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "accessTokenLifetime",
                    &self.r#access_token_lifetime,
                ),
                to_pulumi_object_field(
                    "allowPkceWithoutClientSecret",
                    &self.r#allow_pkce_without_client_secret,
                ),
                to_pulumi_object_field(
                    "appLauncherUrl",
                    &self.r#app_launcher_url,
                ),
                to_pulumi_object_field(
                    "authType",
                    &self.r#auth_type,
                ),
                to_pulumi_object_field(
                    "clientId",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "clientSecret",
                    &self.r#client_secret,
                ),
                to_pulumi_object_field(
                    "consumerServiceUrl",
                    &self.r#consumer_service_url,
                ),
                to_pulumi_object_field(
                    "customAttributes",
                    &self.r#custom_attributes,
                ),
                to_pulumi_object_field(
                    "customClaims",
                    &self.r#custom_claims,
                ),
                to_pulumi_object_field(
                    "defaultRelayState",
                    &self.r#default_relay_state,
                ),
                to_pulumi_object_field(
                    "grantTypes",
                    &self.r#grant_types,
                ),
                to_pulumi_object_field(
                    "groupFilterRegex",
                    &self.r#group_filter_regex,
                ),
                to_pulumi_object_field(
                    "hybridAndImplicitOptions",
                    &self.r#hybrid_and_implicit_options,
                ),
                to_pulumi_object_field(
                    "idpEntityId",
                    &self.r#idp_entity_id,
                ),
                to_pulumi_object_field(
                    "nameIdFormat",
                    &self.r#name_id_format,
                ),
                to_pulumi_object_field(
                    "nameIdTransformJsonata",
                    &self.r#name_id_transform_jsonata,
                ),
                to_pulumi_object_field(
                    "publicKey",
                    &self.r#public_key,
                ),
                to_pulumi_object_field(
                    "redirectUris",
                    &self.r#redirect_uris,
                ),
                to_pulumi_object_field(
                    "refreshTokenOptions",
                    &self.r#refresh_token_options,
                ),
                to_pulumi_object_field(
                    "samlAttributeTransformJsonata",
                    &self.r#saml_attribute_transform_jsonata,
                ),
                to_pulumi_object_field(
                    "scopes",
                    &self.r#scopes,
                ),
                to_pulumi_object_field(
                    "spEntityId",
                    &self.r#sp_entity_id,
                ),
                to_pulumi_object_field(
                    "ssoEndpoint",
                    &self.r#sso_endpoint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("accessTokenLifetime") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessTokenLifetime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_pkce_without_client_secret: {
                        let field_value = match fields_map.get("allowPkceWithoutClientSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowPkceWithoutClientSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_launcher_url: {
                        let field_value = match fields_map.get("appLauncherUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'appLauncherUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_type: {
                        let field_value = match fields_map.get("authType") {
                            Some(value) => value,
                            None => bail!("Missing field 'authType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("clientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("clientSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumer_service_url: {
                        let field_value = match fields_map.get("consumerServiceUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumerServiceUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_attributes: {
                        let field_value = match fields_map.get("customAttributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'customAttributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_claims: {
                        let field_value = match fields_map.get("customClaims") {
                            Some(value) => value,
                            None => bail!("Missing field 'customClaims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_relay_state: {
                        let field_value = match fields_map.get("defaultRelayState") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultRelayState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grant_types: {
                        let field_value = match fields_map.get("grantTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'grantTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_filter_regex: {
                        let field_value = match fields_map.get("groupFilterRegex") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupFilterRegex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hybrid_and_implicit_options: {
                        let field_value = match fields_map.get("hybridAndImplicitOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'hybridAndImplicitOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_entity_id: {
                        let field_value = match fields_map.get("idpEntityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'idpEntityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_id_format: {
                        let field_value = match fields_map.get("nameIdFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'nameIdFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_id_transform_jsonata: {
                        let field_value = match fields_map.get("nameIdTransformJsonata") {
                            Some(value) => value,
                            None => bail!("Missing field 'nameIdTransformJsonata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_key: {
                        let field_value = match fields_map.get("publicKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_uris: {
                        let field_value = match fields_map.get("redirectUris") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectUris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_token_options: {
                        let field_value = match fields_map.get("refreshTokenOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'refreshTokenOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#saml_attribute_transform_jsonata: {
                        let field_value = match fields_map.get("samlAttributeTransformJsonata") {
                            Some(value) => value,
                            None => bail!("Missing field 'samlAttributeTransformJsonata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("spEntityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'spEntityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_endpoint: {
                        let field_value = match fields_map.get("ssoEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssoEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
