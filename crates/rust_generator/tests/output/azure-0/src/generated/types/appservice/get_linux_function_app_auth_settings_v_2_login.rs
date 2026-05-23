#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxFunctionAppAuthSettingsV2Login {
    /// External URLs that can be redirected to as part of logging in or logging out of the app.
    #[builder(into)]
    #[serde(rename = "allowedExternalRedirectUrls")]
    pub r#allowed_external_redirect_urls: Vec<String>,
    /// The method by which cookies expire.
    #[builder(into)]
    #[serde(rename = "cookieExpirationConvention")]
    pub r#cookie_expiration_convention: String,
    /// The time after the request is made when the session cookie should expire.
    #[builder(into)]
    #[serde(rename = "cookieExpirationTime")]
    pub r#cookie_expiration_time: String,
    /// The endpoint to which logout requests are made.
    #[builder(into)]
    #[serde(rename = "logoutEndpoint")]
    pub r#logout_endpoint: String,
    /// The time after the request is made when the nonce should expire.
    #[builder(into)]
    #[serde(rename = "nonceExpirationTime")]
    pub r#nonce_expiration_time: String,
    /// Are the fragments from the request preserved after the login request is made.
    #[builder(into)]
    #[serde(rename = "preserveUrlFragmentsForLogins")]
    pub r#preserve_url_fragments_for_logins: bool,
    /// The number of hours after session token expiration that a session token can be used to call the token refresh API.
    #[builder(into)]
    #[serde(rename = "tokenRefreshExtensionTime")]
    pub r#token_refresh_extension_time: f64,
    /// Is the Token Store configuration Enabled.
    #[builder(into)]
    #[serde(rename = "tokenStoreEnabled")]
    pub r#token_store_enabled: bool,
    /// The directory path in the App Filesystem in which the tokens are stored.
    #[builder(into)]
    #[serde(rename = "tokenStorePath")]
    pub r#token_store_path: String,
    /// The name of the app setting which contains the SAS URL of the blob storage containing the tokens.
    #[builder(into)]
    #[serde(rename = "tokenStoreSasSettingName")]
    pub r#token_store_sas_setting_name: String,
    /// Is the nonce validated while completing the login flow.
    #[builder(into)]
    #[serde(rename = "validateNonce")]
    pub r#validate_nonce: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxFunctionAppAuthSettingsV2Login {
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
                    "allowedExternalRedirectUrls",
                    &self.r#allowed_external_redirect_urls,
                ),
                to_pulumi_object_field(
                    "cookieExpirationConvention",
                    &self.r#cookie_expiration_convention,
                ),
                to_pulumi_object_field(
                    "cookieExpirationTime",
                    &self.r#cookie_expiration_time,
                ),
                to_pulumi_object_field(
                    "logoutEndpoint",
                    &self.r#logout_endpoint,
                ),
                to_pulumi_object_field(
                    "nonceExpirationTime",
                    &self.r#nonce_expiration_time,
                ),
                to_pulumi_object_field(
                    "preserveUrlFragmentsForLogins",
                    &self.r#preserve_url_fragments_for_logins,
                ),
                to_pulumi_object_field(
                    "tokenRefreshExtensionTime",
                    &self.r#token_refresh_extension_time,
                ),
                to_pulumi_object_field(
                    "tokenStoreEnabled",
                    &self.r#token_store_enabled,
                ),
                to_pulumi_object_field(
                    "tokenStorePath",
                    &self.r#token_store_path,
                ),
                to_pulumi_object_field(
                    "tokenStoreSasSettingName",
                    &self.r#token_store_sas_setting_name,
                ),
                to_pulumi_object_field(
                    "validateNonce",
                    &self.r#validate_nonce,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxFunctionAppAuthSettingsV2Login {
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
                        let field_value = match fields_map.get("allowedExternalRedirectUrls") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedExternalRedirectUrls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_expiration_convention: {
                        let field_value = match fields_map.get("cookieExpirationConvention") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookieExpirationConvention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookie_expiration_time: {
                        let field_value = match fields_map.get("cookieExpirationTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookieExpirationTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logout_endpoint: {
                        let field_value = match fields_map.get("logoutEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'logoutEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nonce_expiration_time: {
                        let field_value = match fields_map.get("nonceExpirationTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'nonceExpirationTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_url_fragments_for_logins: {
                        let field_value = match fields_map.get("preserveUrlFragmentsForLogins") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserveUrlFragmentsForLogins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_refresh_extension_time: {
                        let field_value = match fields_map.get("tokenRefreshExtensionTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenRefreshExtensionTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_store_enabled: {
                        let field_value = match fields_map.get("tokenStoreEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenStoreEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_store_path: {
                        let field_value = match fields_map.get("tokenStorePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenStorePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_store_sas_setting_name: {
                        let field_value = match fields_map.get("tokenStoreSasSettingName") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenStoreSasSettingName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate_nonce: {
                        let field_value = match fields_map.get("validateNonce") {
                            Some(value) => value,
                            None => bail!("Missing field 'validateNonce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
