#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppAuthSetting {
    /// A `active_directory` block as defined above.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingActiveDirectory>,
    /// A `additional_login_parameters` block as defined above.
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
    /// Is the Backup enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A `facebook` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebooks")]
    pub r#facebooks: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingFacebook>,
    /// A `github` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingGithub>,
    /// A `google` block as defined below.
    #[builder(into)]
    #[serde(rename = "googles")]
    pub r#googles: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingGoogle>,
    /// The OpenID Connect Issuer URI that represents the entity which issues access tokens for this Windows Web App.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: String,
    /// A `microsoft` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsofts")]
    pub r#microsofts: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingMicrosoft>,
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
    pub r#twitters: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingTwitter>,
    /// The action to take when an unauthenticated client attempts to access the app.
    #[builder(into)]
    #[serde(rename = "unauthenticatedClientAction")]
    pub r#unauthenticated_client_action: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppAuthSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "active_directories",
                    &self.r#active_directories,
                ),
                to_pulumi_object_field(
                    "additional_login_parameters",
                    &self.r#additional_login_parameters,
                ),
                to_pulumi_object_field(
                    "allowed_external_redirect_urls",
                    &self.r#allowed_external_redirect_urls,
                ),
                to_pulumi_object_field(
                    "default_provider",
                    &self.r#default_provider,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "facebooks",
                    &self.r#facebooks,
                ),
                to_pulumi_object_field(
                    "githubs",
                    &self.r#githubs,
                ),
                to_pulumi_object_field(
                    "googles",
                    &self.r#googles,
                ),
                to_pulumi_object_field(
                    "issuer",
                    &self.r#issuer,
                ),
                to_pulumi_object_field(
                    "microsofts",
                    &self.r#microsofts,
                ),
                to_pulumi_object_field(
                    "runtime_version",
                    &self.r#runtime_version,
                ),
                to_pulumi_object_field(
                    "token_refresh_extension_hours",
                    &self.r#token_refresh_extension_hours,
                ),
                to_pulumi_object_field(
                    "token_store_enabled",
                    &self.r#token_store_enabled,
                ),
                to_pulumi_object_field(
                    "twitters",
                    &self.r#twitters,
                ),
                to_pulumi_object_field(
                    "unauthenticated_client_action",
                    &self.r#unauthenticated_client_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppAuthSetting {
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
