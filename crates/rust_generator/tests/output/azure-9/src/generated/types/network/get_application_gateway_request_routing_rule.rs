#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayRequestRoutingRule {
    /// The ID of the associated Backend Address Pool.
    #[builder(into)]
    pub r#backend_address_pool_id: String,
    /// The Name of the Backend Address Pool which is used for this Routing Rule.
    #[builder(into)]
    pub r#backend_address_pool_name: String,
    /// The ID of the associated Backend HTTP Settings Configuration.
    #[builder(into)]
    pub r#backend_http_settings_id: String,
    /// The Name of the Backend HTTP Settings Collection which is used for this Routing Rule.
    #[builder(into)]
    pub r#backend_http_settings_name: String,
    /// The ID of the associated HTTP Listener.
    #[builder(into)]
    pub r#http_listener_id: String,
    /// The Name of the HTTP Listener which is used for this Routing Rule.
    #[builder(into)]
    pub r#http_listener_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    pub r#name: String,
    /// The Priority of this Routing Rule.
    #[builder(into)]
    pub r#priority: i32,
    /// The ID of the associated Redirect Configuration.
    #[builder(into)]
    pub r#redirect_configuration_id: String,
    /// The Name of the Redirect Configuration which is used for this Routing Rule.
    #[builder(into)]
    pub r#redirect_configuration_name: String,
    /// The ID of the associated Rewrite Rule Set.
    #[builder(into)]
    pub r#rewrite_rule_set_id: String,
    /// The Name of the Rewrite Rule Set which is used for this Routing Rule.
    #[builder(into)]
    pub r#rewrite_rule_set_name: String,
    /// The Type of Routing that is used for this Rule.
    #[builder(into)]
    pub r#rule_type: String,
    /// The ID of the associated URL Path Map.
    #[builder(into)]
    pub r#url_path_map_id: String,
    /// The Name of the URL Path Map which is associated with this Routing Rule.
    #[builder(into)]
    pub r#url_path_map_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayRequestRoutingRule {
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
                    "backendAddressPoolId",
                    &self.r#backend_address_pool_id,
                ),
                to_pulumi_object_field(
                    "backendAddressPoolName",
                    &self.r#backend_address_pool_name,
                ),
                to_pulumi_object_field(
                    "backendHttpSettingsId",
                    &self.r#backend_http_settings_id,
                ),
                to_pulumi_object_field(
                    "backendHttpSettingsName",
                    &self.r#backend_http_settings_name,
                ),
                to_pulumi_object_field(
                    "httpListenerId",
                    &self.r#http_listener_id,
                ),
                to_pulumi_object_field(
                    "httpListenerName",
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
                    "redirectConfigurationId",
                    &self.r#redirect_configuration_id,
                ),
                to_pulumi_object_field(
                    "redirectConfigurationName",
                    &self.r#redirect_configuration_name,
                ),
                to_pulumi_object_field(
                    "rewriteRuleSetId",
                    &self.r#rewrite_rule_set_id,
                ),
                to_pulumi_object_field(
                    "rewriteRuleSetName",
                    &self.r#rewrite_rule_set_name,
                ),
                to_pulumi_object_field(
                    "ruleType",
                    &self.r#rule_type,
                ),
                to_pulumi_object_field(
                    "urlPathMapId",
                    &self.r#url_path_map_id,
                ),
                to_pulumi_object_field(
                    "urlPathMapName",
                    &self.r#url_path_map_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayRequestRoutingRule {
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
                        let field_value = match fields_map.get("backendAddressPoolId") {
                            Some(value) => value,
                            None => bail!("Missing field 'backendAddressPoolId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_address_pool_name: {
                        let field_value = match fields_map.get("backendAddressPoolName") {
                            Some(value) => value,
                            None => bail!("Missing field 'backendAddressPoolName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_http_settings_id: {
                        let field_value = match fields_map.get("backendHttpSettingsId") {
                            Some(value) => value,
                            None => bail!("Missing field 'backendHttpSettingsId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_http_settings_name: {
                        let field_value = match fields_map.get("backendHttpSettingsName") {
                            Some(value) => value,
                            None => bail!("Missing field 'backendHttpSettingsName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_listener_id: {
                        let field_value = match fields_map.get("httpListenerId") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpListenerId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_listener_name: {
                        let field_value = match fields_map.get("httpListenerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpListenerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("redirectConfigurationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectConfigurationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_configuration_name: {
                        let field_value = match fields_map.get("redirectConfigurationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectConfigurationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rewrite_rule_set_id: {
                        let field_value = match fields_map.get("rewriteRuleSetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewriteRuleSetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rewrite_rule_set_name: {
                        let field_value = match fields_map.get("rewriteRuleSetName") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewriteRuleSetName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_type: {
                        let field_value = match fields_map.get("ruleType") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_map_id: {
                        let field_value = match fields_map.get("urlPathMapId") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlPathMapId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_path_map_name: {
                        let field_value = match fields_map.get("urlPathMapName") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlPathMapName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
