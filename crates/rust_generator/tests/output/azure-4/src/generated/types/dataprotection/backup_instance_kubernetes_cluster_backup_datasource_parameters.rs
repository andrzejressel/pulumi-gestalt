#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupInstanceKubernetesClusterBackupDatasourceParameters {
    /// Whether to include cluster scope resources during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterScopedResourcesEnabled")]
    pub r#cluster_scoped_resources_enabled: Option<bool>,
    /// Specifies the namespaces to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "excludedNamespaces")]
    pub r#excluded_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "excludedResourceTypes")]
    pub r#excluded_resource_types: Option<Vec<String>>,
    /// Specifies the namespaces to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "includedNamespaces")]
    pub r#included_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "includedResourceTypes")]
    pub r#included_resource_types: Option<Vec<String>>,
    /// Specifies the resources with such label selectors to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "labelSelectors")]
    pub r#label_selectors: Option<Vec<String>>,
    /// Whether to take volume snapshots during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumeSnapshotEnabled")]
    pub r#volume_snapshot_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupInstanceKubernetesClusterBackupDatasourceParameters {
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
                    "cluster_scoped_resources_enabled",
                    &self.r#cluster_scoped_resources_enabled,
                ),
                to_pulumi_object_field(
                    "excluded_namespaces",
                    &self.r#excluded_namespaces,
                ),
                to_pulumi_object_field(
                    "excluded_resource_types",
                    &self.r#excluded_resource_types,
                ),
                to_pulumi_object_field(
                    "included_namespaces",
                    &self.r#included_namespaces,
                ),
                to_pulumi_object_field(
                    "included_resource_types",
                    &self.r#included_resource_types,
                ),
                to_pulumi_object_field(
                    "label_selectors",
                    &self.r#label_selectors,
                ),
                to_pulumi_object_field(
                    "volume_snapshot_enabled",
                    &self.r#volume_snapshot_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupInstanceKubernetesClusterBackupDatasourceParameters {
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
                    r#cluster_scoped_resources_enabled: {
                        let field_value = match fields_map.get("cluster_scoped_resources_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_scoped_resources_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_namespaces: {
                        let field_value = match fields_map.get("excluded_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_resource_types: {
                        let field_value = match fields_map.get("excluded_resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_namespaces: {
                        let field_value = match fields_map.get("included_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_resource_types: {
                        let field_value = match fields_map.get("included_resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_selectors: {
                        let field_value = match fields_map.get("label_selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_snapshot_enabled: {
                        let field_value = match fields_map.get("volume_snapshot_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_snapshot_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
