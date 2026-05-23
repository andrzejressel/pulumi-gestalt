#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupInstanceKubernetesClusterBackupDatasourceParameters {
    /// Whether to include cluster scope resources during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#cluster_scoped_resources_enabled: Option<bool>,
    /// Specifies the namespaces to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#excluded_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#excluded_resource_types: Option<Vec<String>>,
    /// Specifies the namespaces to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#included_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#included_resource_types: Option<Vec<String>>,
    /// Specifies the resources with such label selectors to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#label_selectors: Option<Vec<String>>,
    /// Whether to take volume snapshots during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#volume_snapshot_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupInstanceKubernetesClusterBackupDatasourceParameters {
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
                    "clusterScopedResourcesEnabled",
                    &self.r#cluster_scoped_resources_enabled,
                ),
                to_pulumi_object_field(
                    "excludedNamespaces",
                    &self.r#excluded_namespaces,
                ),
                to_pulumi_object_field(
                    "excludedResourceTypes",
                    &self.r#excluded_resource_types,
                ),
                to_pulumi_object_field(
                    "includedNamespaces",
                    &self.r#included_namespaces,
                ),
                to_pulumi_object_field(
                    "includedResourceTypes",
                    &self.r#included_resource_types,
                ),
                to_pulumi_object_field(
                    "labelSelectors",
                    &self.r#label_selectors,
                ),
                to_pulumi_object_field(
                    "volumeSnapshotEnabled",
                    &self.r#volume_snapshot_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("clusterScopedResourcesEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'clusterScopedResourcesEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_namespaces: {
                        let field_value = match fields_map.get("excludedNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludedNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_resource_types: {
                        let field_value = match fields_map.get("excludedResourceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludedResourceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_namespaces: {
                        let field_value = match fields_map.get("includedNamespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'includedNamespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_resource_types: {
                        let field_value = match fields_map.get("includedResourceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'includedResourceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_selectors: {
                        let field_value = match fields_map.get("labelSelectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'labelSelectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_snapshot_enabled: {
                        let field_value = match fields_map.get("volumeSnapshotEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeSnapshotEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
