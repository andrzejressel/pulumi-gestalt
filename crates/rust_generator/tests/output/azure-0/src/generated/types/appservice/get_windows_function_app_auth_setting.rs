#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsFunctionAppAuthSetting {
    /// A `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingActiveDirectory>,
    /// A map of Login Parameters to send to the OpenID Connect authorization endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: std::collections::HashMap<String, String>,
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Vec<String>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: String,
    /// Is the Backup Job enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebooks")]
    pub r#facebooks: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingFacebook>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingGithub>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "googles")]
    pub r#googles: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingGoogle>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsofts")]
    pub r#microsofts: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingMicrosoft>,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: f64,
    /// Is the Token Store configuration Enabled.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: bool,
    /// A `twitter` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitters")]
    pub r#twitters: Vec<super::super::types::appservice::GetWindowsFunctionAppAuthSettingTwitter>,
    /// The action to take when an unauthenticated client attempts to access the app.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsFunctionAppAuthSetting {
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
                "active_directories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_directories,
                )
                .await,
            );
            map.insert(
                "additional_login_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_login_parameters,
                )
                .await,
            );
            map.insert(
                "allowed_external_redirect_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_external_redirect_urls,
                )
                .await,
            );
            map.insert(
                "default_provider".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_provider,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "facebooks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#facebooks,
                )
                .await,
            );
            map.insert(
                "githubs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#githubs,
                )
                .await,
            );
            map.insert(
                "googles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#googles,
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
                "microsofts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#microsofts,
                )
                .await,
            );
            map.insert(
                "runtime_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#runtime_version,
                )
                .await,
            );
            map.insert(
                "token_refresh_extension_hours".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_refresh_extension_hours,
                )
                .await,
            );
            map.insert(
                "token_store_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_store_enabled,
                )
                .await,
            );
            map.insert(
                "twitters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#twitters,
                )
                .await,
            );
            map.insert(
                "unauthenticated_client_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unauthenticated_client_action,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsFunctionAppAuthSetting {
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
                    r#active_directories: {
                        let field_value = match fields_map.get("active_directories") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_login_parameters: {
                        let field_value = match fields_map.get("additional_login_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_login_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_external_redirect_urls: {
                        let field_value = match fields_map.get("allowed_external_redirect_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_external_redirect_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_provider: {
                        let field_value = match fields_map.get("default_provider") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_provider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#facebooks: {
                        let field_value = match fields_map.get("facebooks") {
                            Some(value) => value,
                            None => bail!("Missing field 'facebooks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#githubs: {
                        let field_value = match fields_map.get("githubs") {
                            Some(value) => value,
                            None => bail!("Missing field 'githubs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#googles: {
                        let field_value = match fields_map.get("googles") {
                            Some(value) => value,
                            None => bail!("Missing field 'googles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#microsofts: {
                        let field_value = match fields_map.get("microsofts") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsofts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_version: {
                        let field_value = match fields_map.get("runtime_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_refresh_extension_hours: {
                        let field_value = match fields_map.get("token_refresh_extension_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_refresh_extension_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_store_enabled: {
                        let field_value = match fields_map.get("token_store_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_store_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#twitters: {
                        let field_value = match fields_map.get("twitters") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unauthenticated_client_action: {
                        let field_value = match fields_map.get("unauthenticated_client_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'unauthenticated_client_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
