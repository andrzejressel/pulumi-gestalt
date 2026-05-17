#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AwsNodePoolKubeletConfig {
    /// Whether or not to enable CPU CFS quota. Defaults to true.
    #[builder(into)]
    #[serde(rename = "cpuCfsQuota")]
    pub r#cpu_cfs_quota: Option<bool>,
    /// Optional. The CPU CFS quota period to use for the node. Defaults to "100ms".
    #[builder(into)]
    #[serde(rename = "cpuCfsQuotaPeriod")]
    pub r#cpu_cfs_quota_period: Option<String>,
    /// The CpuManagerPolicy to use for the node. Defaults to "none".
    #[builder(into)]
    #[serde(rename = "cpuManagerPolicy")]
    pub r#cpu_manager_policy: Option<String>,
    /// Optional. The maximum number of PIDs in each pod running on the node. The limit scales automatically based on underlying machine size if left unset.
    #[builder(into)]
    #[serde(rename = "podPidsLimit")]
    pub r#pod_pids_limit: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AwsNodePoolKubeletConfig {
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AwsNodePoolKubeletConfig {
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
