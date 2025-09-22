#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayRequestRoutingRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolId")]
    pub r#backend_address_pool_id: String,
    /// The Name of the Backend Address Pool which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "backendAddressPoolName")]
    pub r#backend_address_pool_name: String,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsId")]
    pub r#backend_http_settings_id: String,
    /// The Name of the Backend HTTP Settings Collection which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "backendHttpSettingsName")]
    pub r#backend_http_settings_name: String,
    /// The ID of the associated HTTP Listener.
    #[builder(into)]
    #[serde(rename = "httpListenerId")]
    pub r#http_listener_id: String,
    /// The Name of the HTTP Listener which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "httpListenerName")]
    pub r#http_listener_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Priority of this Routing Rule.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The ID of the associated Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationId")]
    pub r#redirect_configuration_id: String,
    /// The Name of the Redirect Configuration which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "redirectConfigurationName")]
    pub r#redirect_configuration_name: String,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetId")]
    pub r#rewrite_rule_set_id: String,
    /// The Name of the Rewrite Rule Set which is used for this Routing Rule.
    #[builder(into)]
    #[serde(rename = "rewriteRuleSetName")]
    pub r#rewrite_rule_set_name: String,
    /// The Type of Routing that is used for this Rule.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: String,
    /// The ID of the associated URL Path Map.
    #[builder(into)]
    #[serde(rename = "urlPathMapId")]
    pub r#url_path_map_id: String,
    /// The Name of the URL Path Map which is associated with this Routing Rule.
    #[builder(into)]
    #[serde(rename = "urlPathMapName")]
    pub r#url_path_map_name: String,
}
