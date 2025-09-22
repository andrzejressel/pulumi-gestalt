#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
