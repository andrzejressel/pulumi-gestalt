#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMonitoringConfig {
    /// Configuration of Advanced Datapath Observability features.
    #[builder(into)]
    #[serde(rename = "advancedDatapathObservabilityConfigs")]
    pub r#advanced_datapath_observability_configs: Vec<super::super::types::container::GetClusterMonitoringConfigAdvancedDatapathObservabilityConfig>,
    /// GKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT, STATEFULSET, WORKLOADS, KUBELET, CADVISOR and DCGM.
    #[builder(into)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Vec<String>,
    /// Configuration for Google Cloud Managed Services for Prometheus.
    #[builder(into)]
    #[serde(rename = "managedPrometheuses")]
    pub r#managed_prometheuses: Vec<super::super::types::container::GetClusterMonitoringConfigManagedPrometheus>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMonitoringConfig {
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
                "advanced_datapath_observability_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_datapath_observability_configs,
                )
                .await,
            );
            map.insert(
                "enable_components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_components,
                )
                .await,
            );
            map.insert(
                "managed_prometheuses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_prometheuses,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterMonitoringConfig {
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
                    r#advanced_datapath_observability_configs: {
                        let field_value = match fields_map.get("advanced_datapath_observability_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_datapath_observability_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_components: {
                        let field_value = match fields_map.get("enable_components") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_prometheuses: {
                        let field_value = match fields_map.get("managed_prometheuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_prometheuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
