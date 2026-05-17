#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodeConfigKubeletConfig {
    /// Enable CPU CFS quota enforcement for containers that specify CPU limits.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: bool,
    /// Set the CPU CFS quota period value 'cpu.cfs_period_us'.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: String,
    /// Control the CPU management policy on the node.
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: String,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: String,
    /// Controls the maximum number of processes allowed to run in a pod.
    #[builder(into)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodeConfigKubeletConfig {
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
                    "cpu_cfs_quota",
                    &self.r#cpu_cfs_quota,
                ),
                to_pulumi_object_field(
                    "cpu_cfs_quota_period",
                    &self.r#cpu_cfs_quota_period,
                ),
                to_pulumi_object_field(
                    "cpu_manager_policy",
                    &self.r#cpu_manager_policy,
                ),
                to_pulumi_object_field(
                    "insecure_kubelet_readonly_port_enabled",
                    &self.r#insecure_kubelet_readonly_port_enabled,
                ),
                to_pulumi_object_field(
                    "pod_pids_limit",
                    &self.r#pod_pids_limit,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodeConfigKubeletConfig {
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
                    r#cpu_cfs_quota: {
                        let field_value = match fields_map.get("cpu_cfs_quota") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_cfs_quota' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#insecure_kubelet_readonly_port_enabled: {
                        let field_value = match fields_map.get("insecure_kubelet_readonly_port_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'insecure_kubelet_readonly_port_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_pids_limit: {
                        let field_value = match fields_map.get("pod_pids_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_pids_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
