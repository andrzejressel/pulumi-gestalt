#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMonitoringConfig {
    /// Configuration for Advanced Datapath Monitoring. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advancedDatapathObservabilityConfig")]
    pub r#advanced_datapath_observability_config: Option<Box<super::super::types::container::ClusterMonitoringConfigAdvancedDatapathObservabilityConfig>>,
    /// The GKE components exposing metrics. Supported values include: `SYSTEM_COMPONENTS`, `APISERVER`, `SCHEDULER`, `CONTROLLER_MANAGER`, `STORAGE`, `HPA`, `POD`, `DAEMONSET`, `DEPLOYMENT`, `STATEFULSET`, `KUBELET`, `CADVISOR` and `DCGM`. In beta provider, `WORKLOADS` is supported on top of those 12 values. (`WORKLOADS` is deprecated and removed in GKE 1.24.) `KUBELET` and `CADVISOR` are only supported in GKE 1.29.3-gke.1093000 and above.
    #[builder(into)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Option<Vec<String>>,
    /// Configuration for Managed Service for Prometheus. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "managedPrometheus")]
    pub r#managed_prometheus: Option<Box<super::super::types::container::ClusterMonitoringConfigManagedPrometheus>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMonitoringConfig {
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
                "advanced_datapath_observability_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_datapath_observability_config,
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
                "managed_prometheus".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_prometheus,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMonitoringConfig {
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
                    r#advanced_datapath_observability_config: {
                        let field_value = match fields_map.get("advanced_datapath_observability_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_datapath_observability_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#managed_prometheus: {
                        let field_value = match fields_map.get("managed_prometheus") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_prometheus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
