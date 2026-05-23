#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppAuthSettingsV2 {
    /// An `active_directory_v2` block as defined below.
    #[builder(into)]
    pub r#active_directory_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2ActiveDirectoryV2>>,
    /// An `apple_v2` block as defined below.
    #[builder(into)]
    pub r#apple_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2AppleV2>>,
    /// Should the AuthV2 Settings be enabled. Defaults to `false`.
    #[builder(into)]
    pub r#auth_enabled: Option<bool>,
    /// An `azure_static_web_app_v2` block as defined below.
    #[builder(into)]
    pub r#azure_static_web_app_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2AzureStaticWebAppV2>>,
    /// The path to the App Auth settings.
    /// 
    /// > **Note:** Relative Paths are evaluated from the Site Root directory.
    #[builder(into)]
    pub r#config_file_path: Option<String>,
    /// Zero or more `custom_oidc_v2` blocks as defined below.
    #[builder(into)]
    pub r#custom_oidc_v_2_s: Option<Vec<super::super::types::appservice::LinuxWebAppAuthSettingsV2CustomOidcV2>>,
    /// The Default Authentication Provider to use when the `unauthenticated_action` is set to `RedirectToLoginPage`. Possible values include: `apple`, `azureactivedirectory`, `facebook`, `github`, `google`, `twitter` and the `name` of your `custom_oidc_v2` provider.
    /// 
    /// > **NOTE:** Whilst any value will be accepted by the API for `default_provider`, it can leave the app in an unusable state if this value does not correspond to the name of a known provider (either built-in value, or custom_oidc name) as it is used to build the auth endpoint URI.
    #[builder(into)]
    pub r#default_provider: Option<String>,
    /// The paths which should be excluded from the `unauthenticated_action` when it is set to `RedirectToLoginPage`.
    /// 
    /// > **NOTE:** This list should be used instead of setting `WEBSITE_WARMUP_PATH` in `app_settings` as it takes priority.
    #[builder(into)]
    pub r#excluded_paths: Option<Vec<String>>,
    /// A `facebook_v2` block as defined below.
    #[builder(into)]
    pub r#facebook_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2FacebookV2>>,
    /// The convention used to determine the url of the request made. Possible values include `NoProxy`, `Standard`, `Custom`. Defaults to `NoProxy`.
    #[builder(into)]
    pub r#forward_proxy_convention: Option<String>,
    /// The name of the custom header containing the host of the request.
    #[builder(into)]
    pub r#forward_proxy_custom_host_header_name: Option<String>,
    /// The name of the custom header containing the scheme of the request.
    #[builder(into)]
    pub r#forward_proxy_custom_scheme_header_name: Option<String>,
    /// A `github_v2` block as defined below.
    #[builder(into)]
    pub r#github_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2GithubV2>>,
    /// A `google_v2` block as defined below.
    #[builder(into)]
    pub r#google_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2GoogleV2>>,
    /// The prefix that should precede all the authentication and authorisation paths. Defaults to `/.auth`.
    #[builder(into)]
    pub r#http_route_api_prefix: Option<String>,
    /// A `login` block as defined below.
    #[builder(into)]
    pub r#login: Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2Login>,
    /// A `microsoft_v2` block as defined below.
    #[builder(into)]
    pub r#microsoft_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2MicrosoftV2>>,
    /// Should the authentication flow be used for all requests.
    #[builder(into)]
    pub r#require_authentication: Option<bool>,
    /// Should HTTPS be required on connections? Defaults to `true`.
    #[builder(into)]
    pub r#require_https: Option<bool>,
    /// The Runtime Version of the Authentication and Authorisation feature of this App. Defaults to `~1`.
    #[builder(into)]
    pub r#runtime_version: Option<String>,
    /// A `twitter_v2` block as defined below.
    #[builder(into)]
    pub r#twitter_v_2: Option<Box<super::super::types::appservice::LinuxWebAppAuthSettingsV2TwitterV2>>,
    /// The action to take for requests made without authentication. Possible values include `RedirectToLoginPage`, `AllowAnonymous`, `Return401`, and `Return403`. Defaults to `RedirectToLoginPage`.
    #[builder(into)]
    pub r#unauthenticated_action: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxWebAppAuthSettingsV2 {
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
                    "activeDirectoryV2",
                    &self.r#active_directory_v_2,
                ),
                to_pulumi_object_field(
                    "appleV2",
                    &self.r#apple_v_2,
                ),
                to_pulumi_object_field(
                    "authEnabled",
                    &self.r#auth_enabled,
                ),
                to_pulumi_object_field(
                    "azureStaticWebAppV2",
                    &self.r#azure_static_web_app_v_2,
                ),
                to_pulumi_object_field(
                    "configFilePath",
                    &self.r#config_file_path,
                ),
                to_pulumi_object_field(
                    "customOidcV2s",
                    &self.r#custom_oidc_v_2_s,
                ),
                to_pulumi_object_field(
                    "defaultProvider",
                    &self.r#default_provider,
                ),
                to_pulumi_object_field(
                    "excludedPaths",
                    &self.r#excluded_paths,
                ),
                to_pulumi_object_field(
                    "facebookV2",
                    &self.r#facebook_v_2,
                ),
                to_pulumi_object_field(
                    "forwardProxyConvention",
                    &self.r#forward_proxy_convention,
                ),
                to_pulumi_object_field(
                    "forwardProxyCustomHostHeaderName",
                    &self.r#forward_proxy_custom_host_header_name,
                ),
                to_pulumi_object_field(
                    "forwardProxyCustomSchemeHeaderName",
                    &self.r#forward_proxy_custom_scheme_header_name,
                ),
                to_pulumi_object_field(
                    "githubV2",
                    &self.r#github_v_2,
                ),
                to_pulumi_object_field(
                    "googleV2",
                    &self.r#google_v_2,
                ),
                to_pulumi_object_field(
                    "httpRouteApiPrefix",
                    &self.r#http_route_api_prefix,
                ),
                to_pulumi_object_field(
                    "login",
                    &self.r#login,
                ),
                to_pulumi_object_field(
                    "microsoftV2",
                    &self.r#microsoft_v_2,
                ),
                to_pulumi_object_field(
                    "requireAuthentication",
                    &self.r#require_authentication,
                ),
                to_pulumi_object_field(
                    "requireHttps",
                    &self.r#require_https,
                ),
                to_pulumi_object_field(
                    "runtimeVersion",
                    &self.r#runtime_version,
                ),
                to_pulumi_object_field(
                    "twitterV2",
                    &self.r#twitter_v_2,
                ),
                to_pulumi_object_field(
                    "unauthenticatedAction",
                    &self.r#unauthenticated_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxWebAppAuthSettingsV2 {
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
                    r#active_directory_v_2: {
                        let field_value = match fields_map.get("activeDirectoryV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apple_v_2: {
                        let field_value = match fields_map.get("appleV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'appleV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_enabled: {
                        let field_value = match fields_map.get("authEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'authEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_static_web_app_v_2: {
                        let field_value = match fields_map.get("azureStaticWebAppV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureStaticWebAppV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_file_path: {
                        let field_value = match fields_map.get("configFilePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'configFilePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_oidc_v_2_s: {
                        let field_value = match fields_map.get("customOidcV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'customOidcV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_provider: {
                        let field_value = match fields_map.get("defaultProvider") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultProvider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_paths: {
                        let field_value = match fields_map.get("excludedPaths") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludedPaths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#facebook_v_2: {
                        let field_value = match fields_map.get("facebookV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'facebookV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_convention: {
                        let field_value = match fields_map.get("forwardProxyConvention") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwardProxyConvention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_custom_host_header_name: {
                        let field_value = match fields_map.get("forwardProxyCustomHostHeaderName") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwardProxyCustomHostHeaderName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_proxy_custom_scheme_header_name: {
                        let field_value = match fields_map.get("forwardProxyCustomSchemeHeaderName") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwardProxyCustomSchemeHeaderName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#github_v_2: {
                        let field_value = match fields_map.get("githubV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'githubV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_v_2: {
                        let field_value = match fields_map.get("googleV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'googleV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_route_api_prefix: {
                        let field_value = match fields_map.get("httpRouteApiPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpRouteApiPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login: {
                        let field_value = match fields_map.get("login") {
                            Some(value) => value,
                            None => bail!("Missing field 'login' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#microsoft_v_2: {
                        let field_value = match fields_map.get("microsoftV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsoftV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_authentication: {
                        let field_value = match fields_map.get("requireAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_https: {
                        let field_value = match fields_map.get("requireHttps") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireHttps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_version: {
                        let field_value = match fields_map.get("runtimeVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtimeVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#twitter_v_2: {
                        let field_value = match fields_map.get("twitterV2") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitterV2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unauthenticated_action: {
                        let field_value = match fields_map.get("unauthenticatedAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'unauthenticatedAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
