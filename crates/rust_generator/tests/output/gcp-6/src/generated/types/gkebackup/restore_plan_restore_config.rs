#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestorePlanRestoreConfig {
    /// If True, restore all namespaced resources in the Backup.
    /// Setting this field to False will result in an error.
    #[builder(into)]
    #[serde(rename = "allNamespaces")]
    pub r#all_namespaces: Option<bool>,
    /// Defines the behavior for handling the situation where cluster-scoped resources
    /// being restored already exist in the target cluster.
    /// This MUST be set to a value other than `CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED`
    /// if `clusterResourceRestoreScope` is anyting other than `noGroupKinds`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy
    /// for more information on each policy option.
    /// Possible values are: `USE_EXISTING_VERSION`, `USE_BACKUP_VERSION`.
    #[builder(into)]
    #[serde(rename = "clusterResourceConflictPolicy")]
    pub r#cluster_resource_conflict_policy: Option<String>,
    /// Identifies the cluster-scoped resources to restore from the Backup.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "clusterResourceRestoreScope")]
    pub r#cluster_resource_restore_scope: Option<Box<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScope>>,
    /// A list of selected namespaces excluded from restoration.
    /// All namespaces except those in this list will be restored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludedNamespaces")]
    pub r#excluded_namespaces: Option<Box<super::super::types::gkebackup::RestorePlanRestoreConfigExcludedNamespaces>>,
    /// Defines the behavior for handling the situation where sets of namespaced resources
    /// being restored already exist in the target cluster.
    /// This MUST be set to a value other than `NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED`
    /// if the `namespacedResourceRestoreScope` is anything other than `noNamespaces`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode
    /// for more information on each mode.
    /// Possible values are: `DELETE_AND_RESTORE`, `FAIL_ON_CONFLICT`, `MERGE_SKIP_ON_CONFLICT`, `MERGE_REPLACE_VOLUME_ON_CONFLICT`, `MERGE_REPLACE_ON_CONFLICT`.
    #[builder(into)]
    #[serde(rename = "namespacedResourceRestoreMode")]
    pub r#namespaced_resource_restore_mode: Option<String>,
    /// Do not restore any namespaced resources if set to "True".
    /// Specifying this field to "False" is not allowed.
    #[builder(into)]
    #[serde(rename = "noNamespaces")]
    pub r#no_namespaces: Option<bool>,
    /// It contains custom ordering to use on a Restore.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "restoreOrder")]
    pub r#restore_order: Option<Box<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrder>>,
    /// A list of selected ProtectedApplications to restore.
    /// The listed ProtectedApplications and all the resources
    /// to which they refer will be restored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedApplications")]
    pub r#selected_applications: Option<Box<super::super::types::gkebackup::RestorePlanRestoreConfigSelectedApplications>>,
    /// A list of selected namespaces to restore from the Backup.
    /// The listed Namespaces and all resources contained in them will be restored.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedNamespaces")]
    pub r#selected_namespaces: Option<Box<super::super::types::gkebackup::RestorePlanRestoreConfigSelectedNamespaces>>,
    /// A list of transformation rules to be applied against Kubernetes
    /// resources as they are selected for restoration from a Backup.
    /// Rules are executed in order defined - this order matters,
    /// as changes made by a rule may impact the filtering logic of subsequent
    /// rules. An empty list means no transformation will occur.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "transformationRules")]
    pub r#transformation_rules: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigTransformationRule>>,
    /// Specifies the mechanism to be used to restore volume data.
    /// This should be set to a value other than `NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED`
    /// if the `namespacedResourceRestoreScope` is anything other than `noNamespaces`.
    /// If not specified, it will be treated as `NO_VOLUME_DATA_RESTORATION`.
    /// See https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy
    /// for more information on each policy option.
    /// Possible values are: `RESTORE_VOLUME_DATA_FROM_BACKUP`, `REUSE_VOLUME_HANDLE_FROM_BACKUP`, `NO_VOLUME_DATA_RESTORATION`.
    #[builder(into)]
    #[serde(rename = "volumeDataRestorePolicy")]
    pub r#volume_data_restore_policy: Option<String>,
    /// A table that binds volumes by their scope to a restore policy. Bindings
    /// must have a unique scope. Any volumes not scoped in the bindings are
    /// subject to the policy defined in volume_data_restore_policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumeDataRestorePolicyBindings")]
    pub r#volume_data_restore_policy_bindings: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigVolumeDataRestorePolicyBinding>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RestorePlanRestoreConfig {
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
                "all_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#all_namespaces,
                )
                .await,
            );
            map.insert(
                "cluster_resource_conflict_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_resource_conflict_policy,
                )
                .await,
            );
            map.insert(
                "cluster_resource_restore_scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_resource_restore_scope,
                )
                .await,
            );
            map.insert(
                "excluded_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_namespaces,
                )
                .await,
            );
            map.insert(
                "namespaced_resource_restore_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#namespaced_resource_restore_mode,
                )
                .await,
            );
            map.insert(
                "no_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_namespaces,
                )
                .await,
            );
            map.insert(
                "restore_order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#restore_order,
                )
                .await,
            );
            map.insert(
                "selected_applications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selected_applications,
                )
                .await,
            );
            map.insert(
                "selected_namespaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selected_namespaces,
                )
                .await,
            );
            map.insert(
                "transformation_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation_rules,
                )
                .await,
            );
            map.insert(
                "volume_data_restore_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_data_restore_policy,
                )
                .await,
            );
            map.insert(
                "volume_data_restore_policy_bindings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_data_restore_policy_bindings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RestorePlanRestoreConfig {
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
                    r#all_namespaces: {
                        let field_value = match fields_map.get("all_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_resource_conflict_policy: {
                        let field_value = match fields_map.get("cluster_resource_conflict_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_resource_conflict_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_resource_restore_scope: {
                        let field_value = match fields_map.get("cluster_resource_restore_scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_resource_restore_scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#namespaced_resource_restore_mode: {
                        let field_value = match fields_map.get("namespaced_resource_restore_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespaced_resource_restore_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_namespaces: {
                        let field_value = match fields_map.get("no_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restore_order: {
                        let field_value = match fields_map.get("restore_order") {
                            Some(value) => value,
                            None => bail!("Missing field 'restore_order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_applications: {
                        let field_value = match fields_map.get("selected_applications") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_applications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_namespaces: {
                        let field_value = match fields_map.get("selected_namespaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_namespaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation_rules: {
                        let field_value = match fields_map.get("transformation_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_data_restore_policy: {
                        let field_value = match fields_map.get("volume_data_restore_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_data_restore_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_data_restore_policy_bindings: {
                        let field_value = match fields_map.get("volume_data_restore_policy_bindings") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_data_restore_policy_bindings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
