#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRule {
    /// A `cache_expiration_action` block as defined above.
    #[builder(into)]
    #[serde(rename = "cacheExpirationAction")]
    pub r#cache_expiration_action: Option<Box<super::super::types::cdn::EndpointDeliveryRuleCacheExpirationAction>>,
    /// A `cache_key_query_string_action` block as defined above.
    #[builder(into)]
    #[serde(rename = "cacheKeyQueryStringAction")]
    pub r#cache_key_query_string_action: Option<Box<super::super::types::cdn::EndpointDeliveryRuleCacheKeyQueryStringAction>>,
    /// A `cookies_condition` block as defined above.
    #[builder(into)]
    #[serde(rename = "cookiesConditions")]
    pub r#cookies_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleCookiesCondition>>,
    /// A `device_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "deviceCondition")]
    pub r#device_condition: Option<Box<super::super::types::cdn::EndpointDeliveryRuleDeviceCondition>>,
    /// A `http_version_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpVersionConditions")]
    pub r#http_version_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleHttpVersionCondition>>,
    /// A `modify_request_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "modifyRequestHeaderActions")]
    pub r#modify_request_header_actions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleModifyRequestHeaderAction>>,
    /// A `modify_response_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "modifyResponseHeaderActions")]
    pub r#modify_response_header_actions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleModifyResponseHeaderAction>>,
    /// The Name which should be used for this Delivery Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The order used for this rule. The order values should be sequential and begin at `1`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// A `post_arg_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "postArgConditions")]
    pub r#post_arg_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRulePostArgCondition>>,
    /// A `query_string_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "queryStringConditions")]
    pub r#query_string_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleQueryStringCondition>>,
    /// A `remote_address_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "remoteAddressConditions")]
    pub r#remote_address_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRemoteAddressCondition>>,
    /// A `request_body_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestBodyConditions")]
    pub r#request_body_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestBodyCondition>>,
    /// A `request_header_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestHeaderConditions")]
    pub r#request_header_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestHeaderCondition>>,
    /// A `request_method_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestMethodCondition")]
    pub r#request_method_condition: Option<Box<super::super::types::cdn::EndpointDeliveryRuleRequestMethodCondition>>,
    /// A `request_scheme_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestSchemeCondition")]
    pub r#request_scheme_condition: Option<Box<super::super::types::cdn::EndpointDeliveryRuleRequestSchemeCondition>>,
    /// A `request_uri_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestUriConditions")]
    pub r#request_uri_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleRequestUriCondition>>,
    /// A `url_file_extension_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlFileExtensionConditions")]
    pub r#url_file_extension_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlFileExtensionCondition>>,
    /// A `url_file_name_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlFileNameConditions")]
    pub r#url_file_name_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlFileNameCondition>>,
    /// A `url_path_condition` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlPathConditions")]
    pub r#url_path_conditions: Option<Vec<super::super::types::cdn::EndpointDeliveryRuleUrlPathCondition>>,
    /// A `url_redirect_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Option<Box<super::super::types::cdn::EndpointDeliveryRuleUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Option<Box<super::super::types::cdn::EndpointDeliveryRuleUrlRewriteAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointDeliveryRule {
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
                    "cacheExpirationAction",
                    &self.r#cache_expiration_action,
                ),
                to_pulumi_object_field(
                    "cacheKeyQueryStringAction",
                    &self.r#cache_key_query_string_action,
                ),
                to_pulumi_object_field(
                    "cookiesConditions",
                    &self.r#cookies_conditions,
                ),
                to_pulumi_object_field(
                    "deviceCondition",
                    &self.r#device_condition,
                ),
                to_pulumi_object_field(
                    "httpVersionConditions",
                    &self.r#http_version_conditions,
                ),
                to_pulumi_object_field(
                    "modifyRequestHeaderActions",
                    &self.r#modify_request_header_actions,
                ),
                to_pulumi_object_field(
                    "modifyResponseHeaderActions",
                    &self.r#modify_response_header_actions,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "postArgConditions",
                    &self.r#post_arg_conditions,
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
                    "requestMethodCondition",
                    &self.r#request_method_condition,
                ),
                to_pulumi_object_field(
                    "requestSchemeCondition",
                    &self.r#request_scheme_condition,
                ),
                to_pulumi_object_field(
                    "requestUriConditions",
                    &self.r#request_uri_conditions,
                ),
                to_pulumi_object_field(
                    "urlFileExtensionConditions",
                    &self.r#url_file_extension_conditions,
                ),
                to_pulumi_object_field(
                    "urlFileNameConditions",
                    &self.r#url_file_name_conditions,
                ),
                to_pulumi_object_field(
                    "urlPathConditions",
                    &self.r#url_path_conditions,
                ),
                to_pulumi_object_field(
                    "urlRedirectAction",
                    &self.r#url_redirect_action,
                ),
                to_pulumi_object_field(
                    "urlRewriteAction",
                    &self.r#url_rewrite_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointDeliveryRule {
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
                    r#cache_expiration_action: {
                        let field_value = match fields_map.get("cacheExpirationAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheExpirationAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_key_query_string_action: {
                        let field_value = match fields_map.get("cacheKeyQueryStringAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'cacheKeyQueryStringAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#device_condition: {
                        let field_value = match fields_map.get("deviceCondition") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceCondition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#modify_request_header_actions: {
                        let field_value = match fields_map.get("modifyRequestHeaderActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'modifyRequestHeaderActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modify_response_header_actions: {
                        let field_value = match fields_map.get("modifyResponseHeaderActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'modifyResponseHeaderActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#post_arg_conditions: {
                        let field_value = match fields_map.get("postArgConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'postArgConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#request_method_condition: {
                        let field_value = match fields_map.get("requestMethodCondition") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestMethodCondition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_scheme_condition: {
                        let field_value = match fields_map.get("requestSchemeCondition") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestSchemeCondition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#url_file_extension_conditions: {
                        let field_value = match fields_map.get("urlFileExtensionConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlFileExtensionConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_file_name_conditions: {
                        let field_value = match fields_map.get("urlFileNameConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlFileNameConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#url_redirect_action: {
                        let field_value = match fields_map.get("urlRedirectAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlRedirectAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_rewrite_action: {
                        let field_value = match fields_map.get("urlRewriteAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlRewriteAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
