#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDataSources {
    /// A `data_import` block as defined above.
    #[builder(into)]
    #[serde(rename = "dataImport")]
    pub r#data_import: Option<Box<super::super::types::monitoring::DataCollectionRuleDataSourcesDataImport>>,
    /// One or more `extension` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "extensions")]
    pub r#extensions: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesExtension>>,
    /// One or more `iis_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "iisLogs")]
    pub r#iis_logs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesIisLog>>,
    /// One or more `log_file` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "logFiles")]
    pub r#log_files: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesLogFile>>,
    /// One or more `performance_counter` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "performanceCounters")]
    pub r#performance_counters: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPerformanceCounter>>,
    /// One or more `platform_telemetry` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "platformTelemetries")]
    pub r#platform_telemetries: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPlatformTelemetry>>,
    /// One or more `prometheus_forwarder` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "prometheusForwarders")]
    pub r#prometheus_forwarders: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPrometheusForwarder>>,
    /// One or more `syslog` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "syslogs")]
    pub r#syslogs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesSyslog>>,
    /// One or more `windows_event_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsEventLogs")]
    pub r#windows_event_logs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesWindowsEventLog>>,
    /// One or more `windows_firewall_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsFirewallLogs")]
    pub r#windows_firewall_logs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesWindowsFirewallLog>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCollectionRuleDataSources {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "data_import",
                    &self.r#data_import,
                ),
                to_pulumi_object_field(
                    "extensions",
                    &self.r#extensions,
                ),
                to_pulumi_object_field(
                    "iis_logs",
                    &self.r#iis_logs,
                ),
                to_pulumi_object_field(
                    "log_files",
                    &self.r#log_files,
                ),
                to_pulumi_object_field(
                    "performance_counters",
                    &self.r#performance_counters,
                ),
                to_pulumi_object_field(
                    "platform_telemetries",
                    &self.r#platform_telemetries,
                ),
                to_pulumi_object_field(
                    "prometheus_forwarders",
                    &self.r#prometheus_forwarders,
                ),
                to_pulumi_object_field(
                    "syslogs",
                    &self.r#syslogs,
                ),
                to_pulumi_object_field(
                    "windows_event_logs",
                    &self.r#windows_event_logs,
                ),
                to_pulumi_object_field(
                    "windows_firewall_logs",
                    &self.r#windows_firewall_logs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCollectionRuleDataSources {
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
                    r#data_import: {
                        let field_value = match fields_map.get("data_import") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_import' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extensions: {
                        let field_value = match fields_map.get("extensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'extensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iis_logs: {
                        let field_value = match fields_map.get("iis_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'iis_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_files: {
                        let field_value = match fields_map.get("log_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#performance_counters: {
                        let field_value = match fields_map.get("performance_counters") {
                            Some(value) => value,
                            None => bail!("Missing field 'performance_counters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform_telemetries: {
                        let field_value = match fields_map.get("platform_telemetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform_telemetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prometheus_forwarders: {
                        let field_value = match fields_map.get("prometheus_forwarders") {
                            Some(value) => value,
                            None => bail!("Missing field 'prometheus_forwarders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#syslogs: {
                        let field_value = match fields_map.get("syslogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'syslogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_event_logs: {
                        let field_value = match fields_map.get("windows_event_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_event_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_firewall_logs: {
                        let field_value = match fields_map.get("windows_firewall_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_firewall_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
