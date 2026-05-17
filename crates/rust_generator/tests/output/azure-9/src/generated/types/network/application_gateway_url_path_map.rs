#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayUrlPathMap {
    /// The ID of the Default Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolId")]
    pub r#default_backend_address_pool_id: Option<String>,
    /// The Name of the Default Backend Address Pool which should be used for this URL Path Map. Cannot be set if `default_redirect_configuration_name` is set.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolName")]
    pub r#default_backend_address_pool_name: Option<String>,
    /// The ID of the Default Backend HTTP Settings Collection.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsId")]
    pub r#default_backend_http_settings_id: Option<String>,
    /// The Name of the Default Backend HTTP Settings Collection which should be used for this URL Path Map. Cannot be set if `default_redirect_configuration_name` is set.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsName")]
    pub r#default_backend_http_settings_name: Option<String>,
    /// The ID of the Default Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationId")]
    pub r#default_redirect_configuration_id: Option<String>,
    /// The Name of the Default Redirect Configuration which should be used for this URL Path Map. Cannot be set if either `default_backend_address_pool_name` or `default_backend_http_settings_name` is set.
    /// 
    /// > **NOTE:** Both `default_backend_address_pool_name` and `default_backend_http_settings_name` or `default_redirect_configuration_name` should be specified.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationName")]
    pub r#default_redirect_configuration_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetId")]
    pub r#default_rewrite_rule_set_id: Option<String>,
    /// The Name of the Default Rewrite Rule Set which should be used for this URL Path Map. Only valid for v2 SKUs.
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetName")]
    pub r#default_rewrite_rule_set_name: Option<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The Name of the URL Path Map.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `path_rule` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "pathRules")]
    pub r#path_rules: Vec<super::super::types::network::ApplicationGatewayUrlPathMapPathRule>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewayUrlPathMap {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "default_backend_address_pool_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_backend_address_pool_id,
                )
                .await,
            );
            map.insert(
                "default_backend_address_pool_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_backend_address_pool_name,
                )
                .await,
            );
            map.insert(
                "default_backend_http_settings_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_backend_http_settings_id,
                )
                .await,
            );
            map.insert(
                "default_backend_http_settings_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_backend_http_settings_name,
                )
                .await,
            );
            map.insert(
                "default_redirect_configuration_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_redirect_configuration_id,
                )
                .await,
            );
            map.insert(
                "default_redirect_configuration_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_redirect_configuration_name,
                )
                .await,
            );
            map.insert(
                "default_rewrite_rule_set_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_rewrite_rule_set_id,
                )
                .await,
            );
            map.insert(
                "default_rewrite_rule_set_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_rewrite_rule_set_name,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "path_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_rules,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewayUrlPathMap {
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
                    r#default_backend_address_pool_id: {
                        let field_value = match fields_map.get("default_backend_address_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_backend_address_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_backend_address_pool_name: {
                        let field_value = match fields_map.get("default_backend_address_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_backend_address_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_backend_http_settings_id: {
                        let field_value = match fields_map.get("default_backend_http_settings_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_backend_http_settings_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_backend_http_settings_name: {
                        let field_value = match fields_map.get("default_backend_http_settings_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_backend_http_settings_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_redirect_configuration_id: {
                        let field_value = match fields_map.get("default_redirect_configuration_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_redirect_configuration_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_redirect_configuration_name: {
                        let field_value = match fields_map.get("default_redirect_configuration_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_redirect_configuration_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_rewrite_rule_set_id: {
                        let field_value = match fields_map.get("default_rewrite_rule_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_rewrite_rule_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_rewrite_rule_set_name: {
                        let field_value = match fields_map.get("default_rewrite_rule_set_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_rewrite_rule_set_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#path_rules: {
                        let field_value = match fields_map.get("path_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
