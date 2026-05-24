#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PagesProjectDeploymentConfigsPreview {
    /// Use latest compatibility date for Pages Functions. Defaults to `false`.
    #[builder(into)]
    pub r#always_use_latest_compatibility_date: Option<bool>,
    /// Compatibility date used for Pages Functions.
    #[builder(into)]
    pub r#compatibility_date: Option<String>,
    /// Compatibility flags used for Pages Functions.
    #[builder(into)]
    pub r#compatibility_flags: Option<Vec<String>>,
    /// D1 Databases used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#d_1_databases: Option<std::collections::BTreeMap<String, String>>,
    /// Durable Object namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#durable_object_namespaces: Option<std::collections::BTreeMap<String, String>>,
    /// Environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#environment_variables: Option<std::collections::BTreeMap<String, String>>,
    /// Fail open used for Pages Functions. Defaults to `false`.
    #[builder(into)]
    pub r#fail_open: Option<bool>,
    /// KV namespaces used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#kv_namespaces: Option<std::collections::BTreeMap<String, String>>,
    /// Configuration for placement in the Cloudflare Pages project.
    #[builder(into)]
    pub r#placement: Option<Box<super::types::PagesProjectDeploymentConfigsPreviewPlacement>>,
    /// R2 Buckets used for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#r_2_buckets: Option<std::collections::BTreeMap<String, String>>,
    /// Encrypted environment variables for Pages Functions. Defaults to `map[]`.
    #[builder(into)]
    pub r#secrets: Option<std::collections::BTreeMap<String, String>>,
    /// Services used for Pages Functions.
    #[builder(into)]
    pub r#service_bindings: Option<Vec<super::types::PagesProjectDeploymentConfigsPreviewServiceBinding>>,
    /// Usage model used for Pages Functions. Available values: `unbound`, `bundled`, `standard`. Defaults to `bundled`.
    #[builder(into)]
    pub r#usage_model: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PagesProjectDeploymentConfigsPreview {
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
                    "alwaysUseLatestCompatibilityDate",
                    &self.r#always_use_latest_compatibility_date,
                ),
                to_pulumi_object_field(
                    "compatibilityDate",
                    &self.r#compatibility_date,
                ),
                to_pulumi_object_field(
                    "compatibilityFlags",
                    &self.r#compatibility_flags,
                ),
                to_pulumi_object_field(
                    "d1Databases",
                    &self.r#d_1_databases,
                ),
                to_pulumi_object_field(
                    "durableObjectNamespaces",
                    &self.r#durable_object_namespaces,
                ),
                to_pulumi_object_field(
                    "environmentVariables",
                    &self.r#environment_variables,
                ),
                to_pulumi_object_field(
                    "failOpen",
                    &self.r#fail_open,
                ),
                to_pulumi_object_field(
                    "kvNamespaces",
                    &self.r#kv_namespaces,
                ),
                to_pulumi_object_field(
                    "placement",
                    &self.r#placement,
                ),
                to_pulumi_object_field(
                    "r2Buckets",
                    &self.r#r_2_buckets,
                ),
                to_pulumi_object_field(
                    "secrets",
                    &self.r#secrets,
                ),
                to_pulumi_object_field(
                    "serviceBindings",
                    &self.r#service_bindings,
                ),
                to_pulumi_object_field(
                    "usageModel",
                    &self.r#usage_model,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PagesProjectDeploymentConfigsPreview {
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
                        let field_value = match fields_map.get("alwaysUseLatestCompatibilityDate") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysUseLatestCompatibilityDate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compatibility_date: {
                        let field_value = match fields_map.get("compatibilityDate") {
                            Some(value) => value,
                            None => bail!("Missing field 'compatibilityDate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compatibility_flags: {
                        let field_value = match fields_map.get("compatibilityFlags") {
                            Some(value) => value,
                            None => bail!("Missing field 'compatibilityFlags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#d_1_databases: {
                        let field_value = match fields_map.get("d1Databases") {
                            Some(value) => value,
                            None => bail!("Missing field 'd1Databases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#durable_object_namespaces: {
                        let field_value = match fields_map.get("durableObjectNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'durableObjectNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environmentVariables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environmentVariables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_open: {
                        let field_value = match fields_map.get("failOpen") {
                            Some(value) => value,
                            None => bail!("Missing field 'failOpen' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kv_namespaces: {
                        let field_value = match fields_map.get("kvNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'kvNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("r2Buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'r2Buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("serviceBindings") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceBindings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usage_model: {
                        let field_value = match fields_map.get("usageModel") {
                            Some(value) => value,
                            None => bail!("Missing field 'usageModel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
