#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CloudVmClusterPropertiesDiagnosticsDataCollectionOptions {
    /// Indicates whether diagnostic collection is enabled for the VM cluster
    #[builder(into)]
    #[serde(rename = "diagnosticsEventsEnabled")]
    pub r#diagnostics_events_enabled: Option<bool>,
    /// Indicates whether health monitoring is enabled for the VM cluster
    #[builder(into)]
    #[serde(rename = "healthMonitoringEnabled")]
    pub r#health_monitoring_enabled: Option<bool>,
    /// Indicates whether incident logs and trace collection are enabled for the VM
    /// cluster
    #[builder(into)]
    #[serde(rename = "incidentLogsEnabled")]
    pub r#incident_logs_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CloudVmClusterPropertiesDiagnosticsDataCollectionOptions {
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
                "diagnostics_events_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#diagnostics_events_enabled,
                )
                .await,
            );
            map.insert(
                "health_monitoring_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#health_monitoring_enabled,
                )
                .await,
            );
            map.insert(
                "incident_logs_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#incident_logs_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CloudVmClusterPropertiesDiagnosticsDataCollectionOptions {
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
                    r#diagnostics_events_enabled: {
                        let field_value = match fields_map.get("diagnostics_events_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'diagnostics_events_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_monitoring_enabled: {
                        let field_value = match fields_map.get("health_monitoring_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_monitoring_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incident_logs_enabled: {
                        let field_value = match fields_map.get("incident_logs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'incident_logs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
