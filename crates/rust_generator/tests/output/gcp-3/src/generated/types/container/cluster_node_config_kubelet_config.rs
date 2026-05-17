#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodeConfigKubeletConfig {
    /// If true, enables CPU CFS quota enforcement for
    /// containers that specify CPU limits.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Option<bool>,
    /// The CPU CFS quota period value. Specified
    /// as a sequence of decimal numbers, each with optional fraction and a unit suffix,
    /// such as `"300ms"`. Valid time units are "ns", "us" (or "µs"), "ms", "s", "m",
    /// "h". The value must be a positive duration.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// The CPU management policy on the node. See
    /// [K8S CPU Management Policies](https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/).
    /// One of `"none"` or `"static"`. If unset (or set to the empty string `""`), the API will treat the field as if set to "none".
    /// Prior to the 6.4.0 this field was marked as required. The workaround for the required field
    /// is setting the empty string `""`, which will function identically to not setting this field.
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Option<String>,
    /// Controls whether the kubelet read-only port is enabled. It is strongly recommended to set this to `FALSE`. Possible values: `TRUE`, `FALSE`.
    #[builder(into)]
    #[serde(rename = "insecureKubeletReadonlyPortEnabled")]
    pub r#insecure_kubelet_readonly_port_enabled: Option<String>,
    /// Controls the maximum number of processes allowed to run in a pod. The value must be greater than or equal to 1024 and less than 4194304.
    #[builder(into)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodeConfigKubeletConfig {
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
                "cpu_cfs_quota".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_cfs_quota,
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
                "insecure_kubelet_readonly_port_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#insecure_kubelet_readonly_port_enabled,
                )
                .await,
            );
            map.insert(
                "pod_pids_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_pids_limit,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodeConfigKubeletConfig {
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
