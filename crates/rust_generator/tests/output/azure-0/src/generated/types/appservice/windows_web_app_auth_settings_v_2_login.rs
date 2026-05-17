#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppAuthSettingsV2Login {
    /// External URLs that can be redirected to as part of logging in or logging out of the app. This is an advanced setting typically only needed by Windows Store application backends.
    /// 
    /// > **Note:** URLs within the current domain are always implicitly allowed.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Option<Vec<String>>,
    /// The method by which cookies expire. Possible values include: `FixedTime`, and `IdentityProviderDerived`. Defaults to `FixedTime`.
    #[builder(into)]
    #[serde(rename = "cookieExpirationConvention")]
    pub r#cookie_expiration_convention: Option<String>,
    /// The time after the request is made when the session cookie should expire. Defaults to `08:00:00`.
    #[builder(into)]
    #[serde(rename = "cookieExpirationTime")]
    pub r#cookie_expiration_time: Option<String>,
    /// The endpoint to which logout requests should be made.
    #[builder(into)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: Option<String>,
    /// The time after the request is made when the nonce should expire. Defaults to `00:05:00`.
    #[builder(into)]
    #[serde(rename = "nonceExpirationTime")]
    pub r#nonce_expiration_time: Option<String>,
    /// Should the fragments from the request be preserved after the login request is made. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "preserveUrlFragmentsForLogins")]
    pub r#preserve_url_fragments_for_logins: Option<bool>,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API. Defaults to `72` hours.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionTime")]
    pub r#token_refresh_extension_time: Option<f64>,
    /// Should the Token Store configuration Enabled. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: Option<bool>,
    /// The directory path in the App Filesystem in which the tokens will be stored.
    #[builder(into)]
    #[serde(rename = "tokenStorePath")]
    pub r#token_store_path: Option<String>,
    /// The name of the app setting which contains the SAS URL of the blob storage containing the tokens.
    #[builder(into)]
    #[serde(rename = "tokenStoreSasSettingName")]
    pub r#token_store_sas_setting_name: Option<String>,
    /// Should the nonce be validated while completing the login flow. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "validateNonce")]
    pub r#validate_nonce: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppAuthSettingsV2Login {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allowed_external_redirect_urls",
                    &self.r#allowed_external_redirect_urls,
                ),
                to_pulumi_object_field(
                    "cookie_expiration_convention",
                    &self.r#cookie_expiration_convention,
                ),
                to_pulumi_object_field(
                    "cookie_expiration_time",
                    &self.r#cookie_expiration_time,
                ),
                to_pulumi_object_field(
                    "logout_endpoint",
                    &self.r#logout_endpoint,
                ),
                to_pulumi_object_field(
                    "nonce_expiration_time",
                    &self.r#nonce_expiration_time,
                ),
                to_pulumi_object_field(
                    "preserve_url_fragments_for_logins",
                    &self.r#preserve_url_fragments_for_logins,
                ),
                to_pulumi_object_field(
                    "token_refresh_extension_time",
                    &self.r#token_refresh_extension_time,
                ),
                to_pulumi_object_field(
                    "token_store_enabled",
                    &self.r#token_store_enabled,
                ),
                to_pulumi_object_field(
                    "token_store_path",
                    &self.r#token_store_path,
                ),
                to_pulumi_object_field(
                    "token_store_sas_setting_name",
                    &self.r#token_store_sas_setting_name,
                ),
                to_pulumi_object_field(
                    "validate_nonce",
                    &self.r#validate_nonce,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppAuthSettingsV2Login {
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
                    r#allowed_external_redirect_urls: {
                        let field_value = match fields_map.get("allowed_external_redirect_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_external_redirect_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_expiration_convention: {
                        let field_value = match fields_map.get("cookie_expiration_convention") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie_expiration_convention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_expiration_time: {
                        let field_value = match fields_map.get("cookie_expiration_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie_expiration_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logout_endpoint: {
                        let field_value = match fields_map.get("logout_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'logout_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nonce_expiration_time: {
                        let field_value = match fields_map.get("nonce_expiration_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'nonce_expiration_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_url_fragments_for_logins: {
                        let field_value = match fields_map.get("preserve_url_fragments_for_logins") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_url_fragments_for_logins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_refresh_extension_time: {
                        let field_value = match fields_map.get("token_refresh_extension_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_refresh_extension_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#token_store_path: {
                        let field_value = match fields_map.get("token_store_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_store_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_store_sas_setting_name: {
                        let field_value = match fields_map.get("token_store_sas_setting_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_store_sas_setting_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate_nonce: {
                        let field_value = match fields_map.get("validate_nonce") {
                            Some(value) => value,
                            None => bail!("Missing field 'validate_nonce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
