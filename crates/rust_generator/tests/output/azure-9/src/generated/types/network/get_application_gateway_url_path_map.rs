#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayUrlPathMap {
    /// The ID of the Default Backend Address Pool.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolId")]
    pub r#default_backend_address_pool_id: String,
    /// The Name of the Default Backend Address Pool which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultBackendAddressPoolName")]
    pub r#default_backend_address_pool_name: String,
    /// The ID of the Default Backend HTTP Settings Collection.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsId")]
    pub r#default_backend_http_settings_id: String,
    /// The Name of the Default Backend HTTP Settings Collection which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultBackendHttpSettingsName")]
    pub r#default_backend_http_settings_name: String,
    /// The ID of the Default Redirect Configuration.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationId")]
    pub r#default_redirect_configuration_id: String,
    /// The Name of the Default Redirect Configuration which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultRedirectConfigurationName")]
    pub r#default_redirect_configuration_name: String,
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetId")]
    pub r#default_rewrite_rule_set_id: String,
    /// The Name of the Default Rewrite Rule Set which is used for this URL Path Map.
    #[builder(into)]
    #[serde(rename = "defaultRewriteRuleSetName")]
    pub r#default_rewrite_rule_set_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `path_rule` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "pathRules")]
    pub r#path_rules: Vec<super::super::types::network::GetApplicationGatewayUrlPathMapPathRule>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayUrlPathMap {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "default_backend_address_pool_id",
                    &self.r#default_backend_address_pool_id,
                ),
                to_pulumi_object_field(
                    "default_backend_address_pool_name",
                    &self.r#default_backend_address_pool_name,
                ),
                to_pulumi_object_field(
                    "default_backend_http_settings_id",
                    &self.r#default_backend_http_settings_id,
                ),
                to_pulumi_object_field(
                    "default_backend_http_settings_name",
                    &self.r#default_backend_http_settings_name,
                ),
                to_pulumi_object_field(
                    "default_redirect_configuration_id",
                    &self.r#default_redirect_configuration_id,
                ),
                to_pulumi_object_field(
                    "default_redirect_configuration_name",
                    &self.r#default_redirect_configuration_name,
                ),
                to_pulumi_object_field(
                    "default_rewrite_rule_set_id",
                    &self.r#default_rewrite_rule_set_id,
                ),
                to_pulumi_object_field(
                    "default_rewrite_rule_set_name",
                    &self.r#default_rewrite_rule_set_name,
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
                    "path_rules",
                    &self.r#path_rules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayUrlPathMap {
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
