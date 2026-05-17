#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxFunctionAppAuthSettingsV2CustomOidcV2 {
    /// The endpoint to make the Authorisation Request as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "authorisationEndpoint")]
    pub r#authorisation_endpoint: Option<String>,
    /// The endpoint that provides the keys necessary to validate the token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "certificationUri")]
    pub r#certification_uri: Option<String>,
    /// The Client Credential Method used.
    #[builder(into)]
    #[serde(rename = "clientCredentialMethod")]
    pub r#client_credential_method: Option<String>,
    /// The ID of the Client to use to authenticate with the Custom OIDC.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The App Setting name that contains the secret for this Custom OIDC Client. This is generated from `name` above and suffixed with `_PROVIDER_AUTHENTICATION_SECRET`.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Option<String>,
    /// The endpoint that issued the Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "issuerEndpoint")]
    pub r#issuer_endpoint: Option<String>,
    /// The name of the Custom OIDC Authentication Provider.
    /// 
    /// > **NOTE:** An `app_setting` matching this value in upper case with the suffix of `_PROVIDER_AUTHENTICATION_SECRET` is required. e.g. `MYOIDC_PROVIDER_AUTHENTICATION_SECRET` for a value of `myoidc`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the claim that contains the users name.
    #[builder(into)]
    #[serde(rename = "nameClaimType")]
    pub r#name_claim_type: Option<String>,
    /// The app setting name that contains the `client_secret` value used for the Custom OIDC Login.
    #[builder(into)]
    #[serde(rename = "openidConfigurationEndpoint")]
    pub r#openid_configuration_endpoint: String,
    /// The list of the scopes that should be requested while authenticating.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    /// The endpoint used to request a Token as supplied by `openid_configuration_endpoint` response.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxFunctionAppAuthSettingsV2CustomOidcV2 {
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
                "authorisation_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorisation_endpoint,
                )
                .await,
            );
            map.insert(
                "certification_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certification_uri,
                )
                .await,
            );
            map.insert(
                "client_credential_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_credential_method,
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
                "client_secret_setting_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_secret_setting_name,
                )
                .await,
            );
            map.insert(
                "issuer_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer_endpoint,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "name_claim_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_claim_type,
                )
                .await,
            );
            map.insert(
                "openid_configuration_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#openid_configuration_endpoint,
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
                "token_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxFunctionAppAuthSettingsV2CustomOidcV2 {
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
                    r#authorisation_endpoint: {
                        let field_value = match fields_map.get("authorisation_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorisation_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certification_uri: {
                        let field_value = match fields_map.get("certification_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'certification_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_credential_method: {
                        let field_value = match fields_map.get("client_credential_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_credential_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#client_secret_setting_name: {
                        let field_value = match fields_map.get("client_secret_setting_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret_setting_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer_endpoint: {
                        let field_value = match fields_map.get("issuer_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_claim_type: {
                        let field_value = match fields_map.get("name_claim_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_claim_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#openid_configuration_endpoint: {
                        let field_value = match fields_map.get("openid_configuration_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'openid_configuration_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#token_endpoint: {
                        let field_value = match fields_map.get("token_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
