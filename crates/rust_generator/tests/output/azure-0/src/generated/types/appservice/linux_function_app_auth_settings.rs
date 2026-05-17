#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxFunctionAppAuthSettings {
    /// An `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectory")]
    pub r#active_directory: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsActiveDirectory>>,
    /// Specifies a map of login Parameters to send to the OpenID Connect authorization endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "additionalLoginParameters")]
    pub r#additional_login_parameters: Option<std::collections::HashMap<String, String>>,
    /// Specifies a list of External URLs that can be redirected to as part of logging in or logging out of the Linux Web App.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Option<Vec<String>>,
    /// The default authentication provider to use when multiple providers are configured. Possible values include: `AzureActiveDirectory`, `Facebook`, `Google`, `MicrosoftAccount`, `Twitter`, `Github`
    /// 
    /// > **NOTE:** This setting is only needed if multiple providers are configured, and the `unauthenticated_client_action` is set to "RedirectToLoginPage".
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Option<String>,
    /// Should the Authentication / Authorization feature be enabled for the Linux Web App?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebook")]
    pub r#facebook: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsFacebook>>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "github")]
    pub r#github: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsGithub>>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "google")]
    pub r#google: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsGoogle>>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Linux Web App.
    /// 
    /// > **NOTE:** When using Azure Active Directory, this value is the URI of the directory tenant, e.g. <https://sts.windows.net/{tenant-guid}/>.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsoft")]
    pub r#microsoft: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsMicrosoft>>,
    /// The RuntimeVersion of the Authentication / Authorization feature in use for the Linux Web App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Option<String>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionHours")]
    pub r#token_refresh_extension_hours: Option<f64>,
    /// Should the Linux Web App durably store platform-specific security tokens that are obtained during login flows? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Option<bool>,
    /// A `twitter` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitter")]
    pub r#twitter: Option<Box<super::super::types::appservice::LinuxFunctionAppAuthSettingsTwitter>>,
    /// The action to take when an unauthenticated client attempts to access the app. Possible values include: `RedirectToLoginPage`, `AllowAnonymous`.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxFunctionAppAuthSettings {
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
                "active_directory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_directory,
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
                "facebook".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#facebook,
                )
                .await,
            );
            map.insert(
                "github".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#github,
                )
                .await,
            );
            map.insert(
                "google".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#google,
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
                "microsoft".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#microsoft,
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
                "twitter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#twitter,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxFunctionAppAuthSettings {
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
                    r#active_directory: {
                        let field_value = match fields_map.get("active_directory") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#facebook: {
                        let field_value = match fields_map.get("facebook") {
                            Some(value) => value,
                            None => bail!("Missing field 'facebook' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#github: {
                        let field_value = match fields_map.get("github") {
                            Some(value) => value,
                            None => bail!("Missing field 'github' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google: {
                        let field_value = match fields_map.get("google") {
                            Some(value) => value,
                            None => bail!("Missing field 'google' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#microsoft: {
                        let field_value = match fields_map.get("microsoft") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsoft' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#twitter: {
                        let field_value = match fields_map.get("twitter") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
