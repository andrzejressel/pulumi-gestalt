#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterDefaultNodePoolKubeletConfig {
    /// Specifies the allow list of unsafe sysctls command or patterns (ending in `*`).
    #[builder(into)]
    pub r#allowed_unsafe_sysctls: Option<Vec<String>>,
    /// Specifies the maximum number of container log files that can be present for a container. must be at least 2.
    #[builder(into)]
    pub r#container_log_max_line: Option<i32>,
    /// Specifies the maximum size (e.g. 10MB) of container log file before it is rotated.
    #[builder(into)]
    pub r#container_log_max_size_mb: Option<i32>,
    /// Is CPU CFS quota enforcement for containers enabled? Defaults to `true`.
    #[builder(into)]
    pub r#cpu_cfs_quota_enabled: Option<bool>,
    /// Specifies the CPU CFS quota period value.
    #[builder(into)]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// Specifies the CPU Manager policy to use. Possible values are `none` and `static`,.
    #[builder(into)]
    pub r#cpu_manager_policy: Option<String>,
    /// Specifies the percent of disk usage above which image garbage collection is always run. Must be between `0` and `100`.
    #[builder(into)]
    pub r#image_gc_high_threshold: Option<i32>,
    /// Specifies the percent of disk usage lower than which image garbage collection is never run. Must be between `0` and `100`.
    #[builder(into)]
    pub r#image_gc_low_threshold: Option<i32>,
    /// Specifies the maximum number of processes per pod.
    #[builder(into)]
    pub r#pod_max_pid: Option<i32>,
    /// Specifies the Topology Manager policy to use. Possible values are `none`, `best-effort`, `restricted` or `single-numa-node`.
    #[builder(into)]
    pub r#topology_manager_policy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterDefaultNodePoolKubeletConfig {
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
                    "allowedUnsafeSysctls",
                    &self.r#allowed_unsafe_sysctls,
                ),
                to_pulumi_object_field(
                    "containerLogMaxLine",
                    &self.r#container_log_max_line,
                ),
                to_pulumi_object_field(
                    "containerLogMaxSizeMb",
                    &self.r#container_log_max_size_mb,
                ),
                to_pulumi_object_field(
                    "cpuCfsQuotaEnabled",
                    &self.r#cpu_cfs_quota_enabled,
                ),
                to_pulumi_object_field(
                    "cpuCfsQuotaPeriod",
                    &self.r#cpu_cfs_quota_period,
                ),
                to_pulumi_object_field(
                    "cpuManagerPolicy",
                    &self.r#cpu_manager_policy,
                ),
                to_pulumi_object_field(
                    "imageGcHighThreshold",
                    &self.r#image_gc_high_threshold,
                ),
                to_pulumi_object_field(
                    "imageGcLowThreshold",
                    &self.r#image_gc_low_threshold,
                ),
                to_pulumi_object_field(
                    "podMaxPid",
                    &self.r#pod_max_pid,
                ),
                to_pulumi_object_field(
                    "topologyManagerPolicy",
                    &self.r#topology_manager_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterDefaultNodePoolKubeletConfig {
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
                        let field_value = match fields_map.get("allowedUnsafeSysctls") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedUnsafeSysctls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_log_max_line: {
                        let field_value = match fields_map.get("containerLogMaxLine") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerLogMaxLine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_log_max_size_mb: {
                        let field_value = match fields_map.get("containerLogMaxSizeMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerLogMaxSizeMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_cfs_quota_enabled: {
                        let field_value = match fields_map.get("cpuCfsQuotaEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuCfsQuotaEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_cfs_quota_period: {
                        let field_value = match fields_map.get("cpuCfsQuotaPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuCfsQuotaPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_manager_policy: {
                        let field_value = match fields_map.get("cpuManagerPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpuManagerPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_gc_high_threshold: {
                        let field_value = match fields_map.get("imageGcHighThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageGcHighThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_gc_low_threshold: {
                        let field_value = match fields_map.get("imageGcLowThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageGcLowThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_max_pid: {
                        let field_value = match fields_map.get("podMaxPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'podMaxPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topology_manager_policy: {
                        let field_value = match fields_map.get("topologyManagerPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'topologyManagerPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
