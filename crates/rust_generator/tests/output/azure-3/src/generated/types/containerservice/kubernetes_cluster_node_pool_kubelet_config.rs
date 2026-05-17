#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNodePoolKubeletConfig {
    /// Specifies the allow list of unsafe sysctls command or patterns (ending in `*`). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "allowedUnsafeSysctls")]
    pub r#allowed_unsafe_sysctls: Option<Vec<String>>,
    /// Specifies the maximum number of container log files that can be present for a container. must be at least 2. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerLogMaxLine")]
    pub r#container_log_max_line: Option<i32>,
    /// Specifies the maximum size (e.g. 10MB) of container log file before it is rotated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerLogMaxSizeMb")]
    pub r#container_log_max_size_mb: Option<i32>,
    /// Is CPU CFS quota enforcement for containers enabled? Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaEnabled")]
    pub r#cpu_cfs_quota_enabled: Option<bool>,
    /// Specifies the CPU CFS quota period value. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// Specifies the CPU Manager policy to use. Possible values are `none` and `static`, Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Option<String>,
    /// Specifies the percent of disk usage above which image garbage collection is always run. Must be between `0` and `100`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "imageGcHighThreshold")]
    pub r#image_gc_high_threshold: Option<i32>,
    /// Specifies the percent of disk usage lower than which image garbage collection is never run. Must be between `0` and `100`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "imageGcLowThreshold")]
    pub r#image_gc_low_threshold: Option<i32>,
    /// Specifies the maximum number of processes per pod. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "podMaxPid")]
    pub r#pod_max_pid: Option<i32>,
    /// Specifies the Topology Manager policy to use. Possible values are `none`, `best-effort`, `restricted` or `single-numa-node`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "topologyManagerPolicy")]
    pub r#topology_manager_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterNodePoolKubeletConfig {
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
                "allowed_unsafe_sysctls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_unsafe_sysctls,
                )
                .await,
            );
            map.insert(
                "container_log_max_line".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_log_max_line,
                )
                .await,
            );
            map.insert(
                "container_log_max_size_mb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_log_max_size_mb,
                )
                .await,
            );
            map.insert(
                "cpu_cfs_quota_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_cfs_quota_enabled,
                )
                .await,
            );
            map.insert(
                "cpu_cfs_quota_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_cfs_quota_period,
                )
                .await,
            );
            map.insert(
                "cpu_manager_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_manager_policy,
                )
                .await,
            );
            map.insert(
                "image_gc_high_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_gc_high_threshold,
                )
                .await,
            );
            map.insert(
                "image_gc_low_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_gc_low_threshold,
                )
                .await,
            );
            map.insert(
                "pod_max_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_max_pid,
                )
                .await,
            );
            map.insert(
                "topology_manager_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topology_manager_policy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterNodePoolKubeletConfig {
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
                    r#allowed_unsafe_sysctls: {
                        let field_value = match fields_map.get("allowed_unsafe_sysctls") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_unsafe_sysctls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_log_max_line: {
                        let field_value = match fields_map.get("container_log_max_line") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_log_max_line' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_log_max_size_mb: {
                        let field_value = match fields_map.get("container_log_max_size_mb") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_log_max_size_mb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_cfs_quota_enabled: {
                        let field_value = match fields_map.get("cpu_cfs_quota_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_cfs_quota_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_cfs_quota_period: {
                        let field_value = match fields_map.get("cpu_cfs_quota_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_cfs_quota_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_manager_policy: {
                        let field_value = match fields_map.get("cpu_manager_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_manager_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_gc_high_threshold: {
                        let field_value = match fields_map.get("image_gc_high_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_gc_high_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_gc_low_threshold: {
                        let field_value = match fields_map.get("image_gc_low_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_gc_low_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_max_pid: {
                        let field_value = match fields_map.get("pod_max_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_max_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topology_manager_policy: {
                        let field_value = match fields_map.get("topology_manager_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'topology_manager_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
