#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppAuthSettingsV2 {
    /// An `active_directory_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "activeDirectoryV2s")]
    pub r#active_directory_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2ActiveDirectoryV2>,
    /// An `apple_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "appleV2s")]
    pub r#apple_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2AppleV2>,
    /// Are the AuthV2 Settings enabled.
    #[builder(into)]
    #[serde(rename = "authEnabled")]
    pub r#auth_enabled: bool,
    /// An `azure_static_web_app_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "azureStaticWebAppV2s")]
    pub r#azure_static_web_app_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2AzureStaticWebAppV2>,
    /// The path to the App Auth settings.
    #[builder(into)]
    #[serde(rename = "configFilePath")]
    pub r#config_file_path: String,
    /// Zero or more `custom_oidc_v2` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customOidcV2s")]
    pub r#custom_oidc_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2CustomOidcV2>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: String,
    /// The paths which should be excluded from the `unauthenticated_action` when it is set to `RedirectToLoginPage`.
    #[builder(into)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Vec<String>,
    /// A `facebook_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "facebookV2s")]
    pub r#facebook_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2FacebookV2>,
    /// The convention used to determine the url of the request made.
    #[builder(into)]
    #[serde(rename = "forwardProxyConvention")]
    pub r#forward_proxy_convention: String,
    /// The name of the custom header containing the host of the request.
    #[builder(into)]
    #[serde(rename = "forwardProxyCustomHostHeaderName")]
    pub r#forward_proxy_custom_host_header_name: String,
    /// The name of the custom header containing the scheme of the request.
    #[builder(into)]
    #[serde(rename = "forwardProxyCustomSchemeHeaderName")]
    pub r#forward_proxy_custom_scheme_header_name: String,
    /// A `github_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "githubV2s")]
    pub r#github_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2GithubV2>,
    /// A `google_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "googleV2s")]
    pub r#google_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2GoogleV2>,
    /// The prefix that should precede all the authentication and authorisation paths.
    #[builder(into)]
    #[serde(rename = "httpRouteApiPrefix")]
    pub r#http_route_api_prefix: String,
    /// A `login` block as defined below.
    #[builder(into)]
    #[serde(rename = "logins")]
    pub r#logins: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2Login>,
    /// A `microsoft_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsoftV2s")]
    pub r#microsoft_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2MicrosoftV2>,
    /// Is the authentication flow used for all requests.
    #[builder(into)]
    #[serde(rename = "requireAuthentication")]
    pub r#require_authentication: bool,
    /// Is HTTPS required on connections?
    #[builder(into)]
    #[serde(rename = "requireHttps")]
    pub r#require_https: bool,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
    /// A `twitter_v2` block as defined below.
    #[builder(into)]
    #[serde(rename = "twitterV2s")]
    pub r#twitter_v_2_s: Vec<super::super::types::appservice::GetWindowsWebAppAuthSettingsV2TwitterV2>,
    /// The action to take for requests made without authentication.
    #[builder(into)]
    #[serde(rename = "unauthenticatedAction")]
    pub r#unauthenticated_action: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppAuthSettingsV2 {
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
                "active_directory_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_directory_v_2_s,
                )
                .await,
            );
            map.insert(
                "apple_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apple_v_2_s,
                )
                .await,
            );
            map.insert(
                "auth_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_enabled,
                )
                .await,
            );
            map.insert(
                "azure_static_web_app_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_static_web_app_v_2_s,
                )
                .await,
            );
            map.insert(
                "config_file_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#config_file_path,
                )
                .await,
            );
            map.insert(
                "custom_oidc_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_oidc_v_2_s,
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
                "excluded_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_paths,
                )
                .await,
            );
            map.insert(
                "facebook_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#facebook_v_2_s,
                )
                .await,
            );
            map.insert(
                "forward_proxy_convention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forward_proxy_convention,
                )
                .await,
            );
            map.insert(
                "forward_proxy_custom_host_header_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forward_proxy_custom_host_header_name,
                )
                .await,
            );
            map.insert(
                "forward_proxy_custom_scheme_header_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forward_proxy_custom_scheme_header_name,
                )
                .await,
            );
            map.insert(
                "github_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#github_v_2_s,
                )
                .await,
            );
            map.insert(
                "google_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#google_v_2_s,
                )
                .await,
            );
            map.insert(
                "http_route_api_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_route_api_prefix,
                )
                .await,
            );
            map.insert(
                "logins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logins,
                )
                .await,
            );
            map.insert(
                "microsoft_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#microsoft_v_2_s,
                )
                .await,
            );
            map.insert(
                "require_authentication".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_authentication,
                )
                .await,
            );
            map.insert(
                "require_https".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_https,
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
                "twitter_v_2_s".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#twitter_v_2_s,
                )
                .await,
            );
            map.insert(
                "unauthenticated_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unauthenticated_action,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppAuthSettingsV2 {
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
                    r#active_directory_v_2_s: {
                        let field_value = match fields_map.get("active_directory_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apple_v_2_s: {
                        let field_value = match fields_map.get("apple_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'apple_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_enabled: {
                        let field_value = match fields_map.get("auth_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_static_web_app_v_2_s: {
                        let field_value = match fields_map.get("azure_static_web_app_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_static_web_app_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_file_path: {
                        let field_value = match fields_map.get("config_file_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_file_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_oidc_v_2_s: {
                        let field_value = match fields_map.get("custom_oidc_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_oidc_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#excluded_paths: {
                        let field_value = match fields_map.get("excluded_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#facebook_v_2_s: {
                        let field_value = match fields_map.get("facebook_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'facebook_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_convention: {
                        let field_value = match fields_map.get("forward_proxy_convention") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_proxy_convention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_custom_host_header_name: {
                        let field_value = match fields_map.get("forward_proxy_custom_host_header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_proxy_custom_host_header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_custom_scheme_header_name: {
                        let field_value = match fields_map.get("forward_proxy_custom_scheme_header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_proxy_custom_scheme_header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#github_v_2_s: {
                        let field_value = match fields_map.get("github_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'github_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_v_2_s: {
                        let field_value = match fields_map.get("google_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_route_api_prefix: {
                        let field_value = match fields_map.get("http_route_api_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_route_api_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logins: {
                        let field_value = match fields_map.get("logins") {
                            Some(value) => value,
                            None => bail!("Missing field 'logins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#microsoft_v_2_s: {
                        let field_value = match fields_map.get("microsoft_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsoft_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_authentication: {
                        let field_value = match fields_map.get("require_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_https: {
                        let field_value = match fields_map.get("require_https") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_https' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#twitter_v_2_s: {
                        let field_value = match fields_map.get("twitter_v_2_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitter_v_2_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unauthenticated_action: {
                        let field_value = match fields_map.get("unauthenticated_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'unauthenticated_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
