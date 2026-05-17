#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationKustomization {
    /// Specifies other kustomizations that this kustomization depends on. This kustomization will not reconcile until all dependencies have completed their reconciliation.
    #[builder(into)]
    #[serde(rename = "dependsOns")]
    pub r#depends_ons: Option<Vec<String>>,
    /// Whether garbage collections of Kubernetes objects created by this kustomization is enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "garbageCollectionEnabled")]
    pub r#garbage_collection_enabled: Option<bool>,
    /// Specifies the name of the kustomization.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the path in the source reference to reconcile on the cluster.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Whether re-creating Kubernetes resources on the cluster is enabled when patching fails due to an immutable field change. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "recreatingEnabled")]
    pub r#recreating_enabled: Option<bool>,
    /// The interval at which to re-reconcile the kustomization on the cluster in the event of failure on reconciliation. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "retryIntervalInSeconds")]
    pub r#retry_interval_in_seconds: Option<i32>,
    /// The interval at which to re-reconcile the kustomization on the cluster. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// The maximum time to attempt to reconcile the kustomization on the cluster. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationKustomization {
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
                "depends_ons".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#depends_ons,
                )
                .await,
            );
            map.insert(
                "garbage_collection_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#garbage_collection_enabled,
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
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "recreating_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recreating_enabled,
                )
                .await,
            );
            map.insert(
                "retry_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "sync_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_in_seconds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FluxConfigurationKustomization {
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
                    r#depends_ons: {
                        let field_value = match fields_map.get("depends_ons") {
                            Some(value) => value,
                            None => bail!("Missing field 'depends_ons' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#garbage_collection_enabled: {
                        let field_value = match fields_map.get("garbage_collection_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'garbage_collection_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recreating_enabled: {
                        let field_value = match fields_map.get("recreating_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'recreating_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_interval_in_seconds: {
                        let field_value = match fields_map.get("retry_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("sync_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
