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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewayRequestRoutingRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "backend_address_pool_id",
                    &self.r#backend_address_pool_id,
                ),
                to_pulumi_object_field(
                    "backend_address_pool_name",
                    &self.r#backend_address_pool_name,
                ),
                to_pulumi_object_field(
                    "backend_http_settings_id",
                    &self.r#backend_http_settings_id,
                ),
                to_pulumi_object_field(
                    "backend_http_settings_name",
                    &self.r#backend_http_settings_name,
                ),
                to_pulumi_object_field(
                    "http_listener_id",
                    &self.r#http_listener_id,
                ),
                to_pulumi_object_field(
                    "http_listener_name",
                    &self.r#http_listener_name,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "redirect_configuration_id",
                    &self.r#redirect_configuration_id,
                ),
                to_pulumi_object_field(
                    "redirect_configuration_name",
                    &self.r#redirect_configuration_name,
                ),
                to_pulumi_object_field(
                    "rewrite_rule_set_id",
                    &self.r#rewrite_rule_set_id,
                ),
                to_pulumi_object_field(
                    "rewrite_rule_set_name",
                    &self.r#rewrite_rule_set_name,
                ),
                to_pulumi_object_field(
                    "rule_type",
                    &self.r#rule_type,
                ),
                to_pulumi_object_field(
                    "url_path_map_id",
                    &self.r#url_path_map_id,
                ),
                to_pulumi_object_field(
                    "url_path_map_name",
                    &self.r#url_path_map_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewayRequestRoutingRule {
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
                    r#backend_address_pool_id: {
                        let field_value = match fields_map.get("backend_address_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_address_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_address_pool_name: {
                        let field_value = match fields_map.get("backend_address_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_address_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_http_settings_id: {
                        let field_value = match fields_map.get("backend_http_settings_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_http_settings_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_http_settings_name: {
                        let field_value = match fields_map.get("backend_http_settings_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_http_settings_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_listener_id: {
                        let field_value = match fields_map.get("http_listener_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_listener_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_listener_name: {
                        let field_value = match fields_map.get("http_listener_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_listener_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_configuration_id: {
                        let field_value = match fields_map.get("redirect_configuration_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_configuration_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_configuration_name: {
                        let field_value = match fields_map.get("redirect_configuration_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_configuration_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rewrite_rule_set_id: {
                        let field_value = match fields_map.get("rewrite_rule_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewrite_rule_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rewrite_rule_set_name: {
                        let field_value = match fields_map.get("rewrite_rule_set_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewrite_rule_set_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_type: {
                        let field_value = match fields_map.get("rule_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_map_id: {
                        let field_value = match fields_map.get("url_path_map_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_path_map_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_map_name: {
                        let field_value = match fields_map.get("url_path_map_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_path_map_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
