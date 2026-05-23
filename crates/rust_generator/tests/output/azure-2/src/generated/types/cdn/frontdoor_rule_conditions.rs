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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "clientPortConditions",
                    &self.r#client_port_conditions,
                ),
                to_pulumi_object_field(
                    "cookiesConditions",
                    &self.r#cookies_conditions,
                ),
                to_pulumi_object_field(
                    "hostNameConditions",
                    &self.r#host_name_conditions,
                ),
                to_pulumi_object_field(
                    "httpVersionConditions",
                    &self.r#http_version_conditions,
                ),
                to_pulumi_object_field(
                    "isDeviceConditions",
                    &self.r#is_device_conditions,
                ),
                to_pulumi_object_field(
                    "postArgsConditions",
                    &self.r#post_args_conditions,
                ),
                to_pulumi_object_field(
                    "queryStringConditions",
                    &self.r#query_string_conditions,
                ),
                to_pulumi_object_field(
                    "remoteAddressConditions",
                    &self.r#remote_address_conditions,
                ),
                to_pulumi_object_field(
                    "requestBodyConditions",
                    &self.r#request_body_conditions,
                ),
                to_pulumi_object_field(
                    "requestHeaderConditions",
                    &self.r#request_header_conditions,
                ),
                to_pulumi_object_field(
                    "requestMethodConditions",
                    &self.r#request_method_conditions,
                ),
                to_pulumi_object_field(
                    "requestSchemeConditions",
                    &self.r#request_scheme_conditions,
                ),
                to_pulumi_object_field(
                    "requestUriConditions",
                    &self.r#request_uri_conditions,
                ),
                to_pulumi_object_field(
                    "serverPortConditions",
                    &self.r#server_port_conditions,
                ),
                to_pulumi_object_field(
                    "socketAddressConditions",
                    &self.r#socket_address_conditions,
                ),
                to_pulumi_object_field(
                    "sslProtocolConditions",
                    &self.r#ssl_protocol_conditions,
                ),
                to_pulumi_object_field(
                    "urlFileExtensionConditions",
                    &self.r#url_file_extension_conditions,
                ),
                to_pulumi_object_field(
                    "urlFilenameConditions",
                    &self.r#url_filename_conditions,
                ),
                to_pulumi_object_field(
                    "urlPathConditions",
                    &self.r#url_path_conditions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("clientPortConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientPortConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cookies_conditions: {
                        let field_value = match fields_map.get("cookiesConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookiesConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name_conditions: {
                        let field_value = match fields_map.get("hostNameConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostNameConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_version_conditions: {
                        let field_value = match fields_map.get("httpVersionConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpVersionConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_device_conditions: {
                        let field_value = match fields_map.get("isDeviceConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'isDeviceConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_args_conditions: {
                        let field_value = match fields_map.get("postArgsConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'postArgsConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_conditions: {
                        let field_value = match fields_map.get("queryStringConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryStringConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_address_conditions: {
                        let field_value = match fields_map.get("remoteAddressConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'remoteAddressConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_body_conditions: {
                        let field_value = match fields_map.get("requestBodyConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestBodyConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_header_conditions: {
                        let field_value = match fields_map.get("requestHeaderConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestHeaderConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_method_conditions: {
                        let field_value = match fields_map.get("requestMethodConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestMethodConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_scheme_conditions: {
                        let field_value = match fields_map.get("requestSchemeConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestSchemeConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_uri_conditions: {
                        let field_value = match fields_map.get("requestUriConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestUriConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_port_conditions: {
                        let field_value = match fields_map.get("serverPortConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverPortConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#socket_address_conditions: {
                        let field_value = match fields_map.get("socketAddressConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'socketAddressConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_protocol_conditions: {
                        let field_value = match fields_map.get("sslProtocolConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslProtocolConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_file_extension_conditions: {
                        let field_value = match fields_map.get("urlFileExtensionConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlFileExtensionConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_filename_conditions: {
                        let field_value = match fields_map.get("urlFilenameConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlFilenameConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_conditions: {
                        let field_value = match fields_map.get("urlPathConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlPathConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
