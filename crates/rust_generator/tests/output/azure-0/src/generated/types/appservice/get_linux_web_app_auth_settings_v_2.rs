#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppAuthSettingsV2 {
    /// An `active_directory_v2` block as defined below.
    #[builder(into)]
    pub r#active_directory_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2ActiveDirectoryV2>,
    /// An `apple_v2` block as defined below.
    #[builder(into)]
    pub r#apple_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2AppleV2>,
    /// Are the AuthV2 Settings enabled.
    #[builder(into)]
    pub r#auth_enabled: bool,
    /// An `azure_static_web_app_v2` block as defined below.
    #[builder(into)]
    pub r#azure_static_web_app_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2AzureStaticWebAppV2>,
    /// The path to the App Auth settings.
    #[builder(into)]
    pub r#config_file_path: String,
    /// Zero or more `custom_oidc_v2` blocks as defined below.
    #[builder(into)]
    pub r#custom_oidc_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2CustomOidcV2>,
    /// The Default Authentication Provider used when more than one Authentication Provider is configured and the `unauthenticated_action` is set to `RedirectToLoginPage`.
    #[builder(into)]
    pub r#default_provider: String,
    /// The paths which should be excluded from the `unauthenticated_action` when it is set to `RedirectToLoginPage`.
    #[builder(into)]
    pub r#excluded_paths: Vec<String>,
    /// A `facebook_v2` block as defined below.
    #[builder(into)]
    pub r#facebook_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2FacebookV2>,
    /// The convention used to determine the url of the request made.
    #[builder(into)]
    pub r#forward_proxy_convention: String,
    /// The name of the custom header containing the host of the request.
    #[builder(into)]
    pub r#forward_proxy_custom_host_header_name: String,
    /// The name of the custom header containing the scheme of the request.
    #[builder(into)]
    pub r#forward_proxy_custom_scheme_header_name: String,
    /// A `github_v2` block as defined below.
    #[builder(into)]
    pub r#github_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2GithubV2>,
    /// A `google_v2` block as defined below.
    #[builder(into)]
    pub r#google_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2GoogleV2>,
    /// The prefix that should precede all the authentication and authorisation paths.
    #[builder(into)]
    pub r#http_route_api_prefix: String,
    /// A `login` block as defined below.
    #[builder(into)]
    pub r#logins: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2Login>,
    /// A `microsoft_v2` block as defined below.
    #[builder(into)]
    pub r#microsoft_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2MicrosoftV2>,
    /// Is the authentication flow used for all requests.
    #[builder(into)]
    pub r#require_authentication: bool,
    /// Is HTTPS required on connections?
    #[builder(into)]
    pub r#require_https: bool,
    /// The Runtime Version of the Authentication and Authorisation feature of this App.
    #[builder(into)]
    pub r#runtime_version: String,
    /// A `twitter_v2` block as defined below.
    #[builder(into)]
    pub r#twitter_v_2_s: Vec<super::super::types::appservice::GetLinuxWebAppAuthSettingsV2TwitterV2>,
    /// The action to take for requests made without authentication.
    #[builder(into)]
    pub r#unauthenticated_action: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxWebAppAuthSettingsV2 {
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
                    "activeDirectoryV2s",
                    &self.r#active_directory_v_2_s,
                ),
                to_pulumi_object_field(
                    "appleV2s",
                    &self.r#apple_v_2_s,
                ),
                to_pulumi_object_field(
                    "authEnabled",
                    &self.r#auth_enabled,
                ),
                to_pulumi_object_field(
                    "azureStaticWebAppV2s",
                    &self.r#azure_static_web_app_v_2_s,
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
                    "facebookV2s",
                    &self.r#facebook_v_2_s,
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
                    "githubV2s",
                    &self.r#github_v_2_s,
                ),
                to_pulumi_object_field(
                    "googleV2s",
                    &self.r#google_v_2_s,
                ),
                to_pulumi_object_field(
                    "httpRouteApiPrefix",
                    &self.r#http_route_api_prefix,
                ),
                to_pulumi_object_field(
                    "logins",
                    &self.r#logins,
                ),
                to_pulumi_object_field(
                    "microsoftV2s",
                    &self.r#microsoft_v_2_s,
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
                    "twitterV2s",
                    &self.r#twitter_v_2_s,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxWebAppAuthSettingsV2 {
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
                        let field_value = match fields_map.get("activeDirectoryV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeDirectoryV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apple_v_2_s: {
                        let field_value = match fields_map.get("appleV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'appleV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#azure_static_web_app_v_2_s: {
                        let field_value = match fields_map.get("azureStaticWebAppV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureStaticWebAppV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#facebook_v_2_s: {
                        let field_value = match fields_map.get("facebookV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'facebookV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#github_v_2_s: {
                        let field_value = match fields_map.get("githubV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'githubV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_v_2_s: {
                        let field_value = match fields_map.get("googleV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'googleV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#logins: {
                        let field_value = match fields_map.get("logins") {
                            Some(value) => value,
                            None => bail!("Missing field 'logins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#microsoft_v_2_s: {
                        let field_value = match fields_map.get("microsoftV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsoftV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#twitter_v_2_s: {
                        let field_value = match fields_map.get("twitterV2s") {
                            Some(value) => value,
                            None => bail!("Missing field 'twitterV2s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
