#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
