#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayRequestRoutingRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolId")]
    pub r#backend_address_pool_id: Option<String>,
    /// The Name of the Backend Address Pool which should be used for this Routing Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolName")]
    pub r#backend_address_pool_name: Option<String>,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsId")]
    pub r#backend_http_settings_id: Option<String>,
    /// The Name of the Backend HTTP Settings Collection which should be used for this Routing Rule. Cannot be set if `redirect_configuration_name` is set.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsName")]
    pub r#backend_http_settings_name: Option<String>,
    /// The ID of the associated HTTP Listener.
    #[builder(into)]
    #[serde(rename = "httpListenerId")]
    pub r#http_listener_id: Option<String>,
    /// The Name of the HTTP Listener which should be used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "httpListenerName")]
    pub r#http_listener_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The Name of this Request Routing Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Rule evaluation order can be dictated by specifying an integer value from `1` to `20000` with `1` being the highest priority and `20000` being the lowest priority.
    /// 
    /// > **NOTE:** `priority` is required when `sku[0].tier` is set to `*_v2`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// The ID of the associated Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationId")]
    pub r#redirect_configuration_id: Option<String>,
    /// The Name of the Redirect Configuration which should be used for this Routing Rule. Cannot be set if either `backend_address_pool_name` or `backend_http_settings_name` is set.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationName")]
    pub r#redirect_configuration_name: Option<String>,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetId")]
    pub r#rewrite_rule_set_id: Option<String>,
    /// The Name of the Rewrite Rule Set which should be used for this Routing Rule. Only valid for v2 SKUs.
    /// 
    /// > **NOTE:** `backend_address_pool_name`, `backend_http_settings_name`, `redirect_configuration_name`, and `rewrite_rule_set_name` are applicable only when `rule_type` is `Basic`.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetName")]
    pub r#rewrite_rule_set_name: Option<String>,
    /// The Type of Routing that should be used for this Rule. Possible values are `Basic` and `PathBasedRouting`.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: String,
    /// The ID of the associated URL Path Map.
    #[builder(into)]
    #[serde(rename = "urlPathMapId")]
    pub r#url_path_map_id: Option<String>,
    /// The Name of the URL Path Map which should be associated with this Routing Rule.
    #[builder(into)]
    #[serde(rename = "urlPathMapName")]
    pub r#url_path_map_name: Option<String>,
}
