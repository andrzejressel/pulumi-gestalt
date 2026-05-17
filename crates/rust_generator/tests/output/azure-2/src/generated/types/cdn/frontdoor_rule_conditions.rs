#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleConditions {
    /// A `client_port_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "clientPortConditions")]
    pub r#client_port_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsClientPortCondition>>,
    /// A `cookies_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "cookiesConditions")]
    pub r#cookies_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsCookiesCondition>>,
    /// A `host_name_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "hostNameConditions")]
    pub r#host_name_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsHostNameCondition>>,
    /// A `http_version_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpVersionConditions")]
    pub r#http_version_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsHttpVersionCondition>>,
    /// A `is_device_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "isDeviceConditions")]
    pub r#is_device_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsIsDeviceCondition>>,
    /// A `post_args_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "postArgsConditions")]
    pub r#post_args_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsPostArgsCondition>>,
    /// A `query_string_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "queryStringConditions")]
    pub r#query_string_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsQueryStringCondition>>,
    /// A `remote_address_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "remoteAddressConditions")]
    pub r#remote_address_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRemoteAddressCondition>>,
    /// A `request_body_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestBodyConditions")]
    pub r#request_body_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRequestBodyCondition>>,
    /// A `request_header_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestHeaderConditions")]
    pub r#request_header_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRequestHeaderCondition>>,
    /// A `request_method_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestMethodConditions")]
    pub r#request_method_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRequestMethodCondition>>,
    /// A `request_scheme_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestSchemeConditions")]
    pub r#request_scheme_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRequestSchemeCondition>>,
    /// A `request_uri_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestUriConditions")]
    pub r#request_uri_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsRequestUriCondition>>,
    /// A `server_port_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "serverPortConditions")]
    pub r#server_port_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsServerPortCondition>>,
    /// A `socket_address_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "socketAddressConditions")]
    pub r#socket_address_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsSocketAddressCondition>>,
    /// A `ssl_protocol_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "sslProtocolConditions")]
    pub r#ssl_protocol_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsSslProtocolCondition>>,
    /// A `url_file_extension_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlFileExtensionConditions")]
    pub r#url_file_extension_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsUrlFileExtensionCondition>>,
    /// A `url_filename_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlFilenameConditions")]
    pub r#url_filename_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsUrlFilenameCondition>>,
    /// A `url_path_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlPathConditions")]
    pub r#url_path_conditions: Option<Vec<super::super::types::cdn::FrontdoorRuleConditionsUrlPathCondition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleConditions {
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
                "client_port_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_port_conditions,
                )
                .await,
            );
            map.insert(
                "cookies_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cookies_conditions,
                )
                .await,
            );
            map.insert(
                "host_name_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_name_conditions,
                )
                .await,
            );
            map.insert(
                "http_version_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_version_conditions,
                )
                .await,
            );
            map.insert(
                "is_device_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_device_conditions,
                )
                .await,
            );
            map.insert(
                "post_args_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#post_args_conditions,
                )
                .await,
            );
            map.insert(
                "query_string_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string_conditions,
                )
                .await,
            );
            map.insert(
                "remote_address_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_address_conditions,
                )
                .await,
            );
            map.insert(
                "request_body_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_body_conditions,
                )
                .await,
            );
            map.insert(
                "request_header_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_header_conditions,
                )
                .await,
            );
            map.insert(
                "request_method_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_method_conditions,
                )
                .await,
            );
            map.insert(
                "request_scheme_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_scheme_conditions,
                )
                .await,
            );
            map.insert(
                "request_uri_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_uri_conditions,
                )
                .await,
            );
            map.insert(
                "server_port_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_port_conditions,
                )
                .await,
            );
            map.insert(
                "socket_address_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#socket_address_conditions,
                )
                .await,
            );
            map.insert(
                "ssl_protocol_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_protocol_conditions,
                )
                .await,
            );
            map.insert(
                "url_file_extension_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_file_extension_conditions,
                )
                .await,
            );
            map.insert(
                "url_filename_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_filename_conditions,
                )
                .await,
            );
            map.insert(
                "url_path_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_path_conditions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRuleConditions {
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
                    r#client_port_conditions: {
                        let field_value = match fields_map.get("client_port_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_port_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookies_conditions: {
                        let field_value = match fields_map.get("cookies_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name_conditions: {
                        let field_value = match fields_map.get("host_name_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_version_conditions: {
                        let field_value = match fields_map.get("http_version_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_version_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_device_conditions: {
                        let field_value = match fields_map.get("is_device_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_device_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_args_conditions: {
                        let field_value = match fields_map.get("post_args_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'post_args_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_conditions: {
                        let field_value = match fields_map.get("query_string_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_address_conditions: {
                        let field_value = match fields_map.get("remote_address_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_address_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_conditions: {
                        let field_value = match fields_map.get("request_body_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_body_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_header_conditions: {
                        let field_value = match fields_map.get("request_header_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_method_conditions: {
                        let field_value = match fields_map.get("request_method_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_method_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_scheme_conditions: {
                        let field_value = match fields_map.get("request_scheme_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_scheme_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_uri_conditions: {
                        let field_value = match fields_map.get("request_uri_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_uri_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_port_conditions: {
                        let field_value = match fields_map.get("server_port_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_port_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#socket_address_conditions: {
                        let field_value = match fields_map.get("socket_address_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'socket_address_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_protocol_conditions: {
                        let field_value = match fields_map.get("ssl_protocol_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_protocol_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_file_extension_conditions: {
                        let field_value = match fields_map.get("url_file_extension_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_file_extension_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_filename_conditions: {
                        let field_value = match fields_map.get("url_filename_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_filename_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_conditions: {
                        let field_value = match fields_map.get("url_path_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_path_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
