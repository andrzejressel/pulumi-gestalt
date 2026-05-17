#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PagesProjectDeploymentConfigsProduction {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "alwaysUseLatestCompatibilityDate")]
    pub r#always_use_latest_compatibility_date: Option<bool>,
    /// Compatibility date used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "compatibilityDate")]
    pub r#compatibility_date: Option<String>,
    /// Compatibility flags used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "compatibilityFlags")]
    pub r#compatibility_flags: Option<Vec<String>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "d1Databases")]
    pub r#d_1_databases: Option<std::collections::HashMap<String, String>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "durableObjectNamespaces")]
    pub r#durable_object_namespaces: Option<std::collections::HashMap<String, String>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "kvNamespaces")]
    pub r#kv_namespaces: Option<std::collections::HashMap<String, String>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[builder(into)]
    #[serde(rename = "placement")]
    pub r#placement: Option<Box<super::types::PagesProjectDeploymentConfigsProductionPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "r2Buckets")]
    pub r#r_2_buckets: Option<std::collections::HashMap<String, String>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<std::collections::HashMap<String, String>>,
    /// Services used for Pages Functions.
    #[builder(into)]
    #[serde(rename = "serviceBindings")]
    pub r#service_bindings: Option<Vec<super::types::PagesProjectDeploymentConfigsProductionServiceBinding>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[builder(into)]
    #[serde(rename = "usageModel")]
    pub r#usage_model: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PagesProjectDeploymentConfigsProduction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "always_use_latest_compatibility_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#always_use_latest_compatibility_date,
                )
                .await,
            );
            map.insert(
                "compatibility_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compatibility_date,
                )
                .await,
            );
            map.insert(
                "compatibility_flags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compatibility_flags,
                )
                .await,
            );
            map.insert(
                "d_1_databases".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#d_1_databases,
                )
                .await,
            );
            map.insert(
                "durable_object_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#durable_object_namespaces,
                )
                .await,
            );
            map.insert(
                "environment_variables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment_variables,
                )
                .await,
            );
            map.insert(
                "fail_open".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fail_open,
                )
                .await,
            );
            map.insert(
                "kv_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kv_namespaces,
                )
                .await,
            );
            map.insert(
                "placement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#placement,
                )
                .await,
            );
            map.insert(
                "r_2_buckets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#r_2_buckets,
                )
                .await,
            );
            map.insert(
                "secrets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets,
                )
                .await,
            );
            map.insert(
                "service_bindings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_bindings,
                )
                .await,
            );
            map.insert(
                "usage_model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#usage_model,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PagesProjectDeploymentConfigsProduction {
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
                    r#always_use_latest_compatibility_date: {
                        let field_value = match fields_map.get("always_use_latest_compatibility_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'always_use_latest_compatibility_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compatibility_date: {
                        let field_value = match fields_map.get("compatibility_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'compatibility_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compatibility_flags: {
                        let field_value = match fields_map.get("compatibility_flags") {
                            Some(value) => value,
                            None => bail!("Missing field 'compatibility_flags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#d_1_databases: {
                        let field_value = match fields_map.get("d_1_databases") {
                            Some(value) => value,
                            None => bail!("Missing field 'd_1_databases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#durable_object_namespaces: {
                        let field_value = match fields_map.get("durable_object_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'durable_object_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_open: {
                        let field_value = match fields_map.get("fail_open") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_open' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kv_namespaces: {
                        let field_value = match fields_map.get("kv_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'kv_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement: {
                        let field_value = match fields_map.get("placement") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#r_2_buckets: {
                        let field_value = match fields_map.get("r_2_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_2_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_bindings: {
                        let field_value = match fields_map.get("service_bindings") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_bindings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usage_model: {
                        let field_value = match fields_map.get("usage_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'usage_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
