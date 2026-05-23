#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataCollectionRuleDataSource {
    /// A `data_import` block as defined above.
    #[builder(into)]
    #[serde(rename = "dataImports")]
    pub r#data_imports: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceDataImport>,
    /// One or more `extension` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "extensions")]
    pub r#extensions: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceExtension>,
    /// One or more `iis_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "iisLogs")]
    pub r#iis_logs: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceIisLog>,
    /// One or more `log_file` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "logFiles")]
    pub r#log_files: Option<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceLogFile>>,
    /// One or more `performance_counter` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "performanceCounters")]
    pub r#performance_counters: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePerformanceCounter>,
    /// One or more `platform_telemetry` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "platformTelemetries")]
    pub r#platform_telemetries: Option<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePlatformTelemetry>>,
    /// One or more `prometheus_forwarder` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "prometheusForwarders")]
    pub r#prometheus_forwarders: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePrometheusForwarder>,
    /// One or more `syslog` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "syslogs")]
    pub r#syslogs: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceSyslog>,
    /// One or more `windows_event_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsEventLogs")]
    pub r#windows_event_logs: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceWindowsEventLog>,
    /// One or more `windows_firewall_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsFirewallLogs")]
    pub r#windows_firewall_logs: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceWindowsFirewallLog>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataCollectionRuleDataSource {
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
                    "dataImports",
                    &self.r#data_imports,
                ),
                to_pulumi_object_field(
                    "extensions",
                    &self.r#extensions,
                ),
                to_pulumi_object_field(
                    "iisLogs",
                    &self.r#iis_logs,
                ),
                to_pulumi_object_field(
                    "logFiles",
                    &self.r#log_files,
                ),
                to_pulumi_object_field(
                    "performanceCounters",
                    &self.r#performance_counters,
                ),
                to_pulumi_object_field(
                    "platformTelemetries",
                    &self.r#platform_telemetries,
                ),
                to_pulumi_object_field(
                    "prometheusForwarders",
                    &self.r#prometheus_forwarders,
                ),
                to_pulumi_object_field(
                    "syslogs",
                    &self.r#syslogs,
                ),
                to_pulumi_object_field(
                    "windowsEventLogs",
                    &self.r#windows_event_logs,
                ),
                to_pulumi_object_field(
                    "windowsFirewallLogs",
                    &self.r#windows_firewall_logs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataCollectionRuleDataSource {
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
                    r#data_imports: {
                        let field_value = match fields_map.get("dataImports") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataImports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("iisLogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'iisLogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_files: {
                        let field_value = match fields_map.get("logFiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'logFiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#performance_counters: {
                        let field_value = match fields_map.get("performanceCounters") {
                            Some(value) => value,
                            None => bail!("Missing field 'performanceCounters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform_telemetries: {
                        let field_value = match fields_map.get("platformTelemetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'platformTelemetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prometheus_forwarders: {
                        let field_value = match fields_map.get("prometheusForwarders") {
                            Some(value) => value,
                            None => bail!("Missing field 'prometheusForwarders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("windowsEventLogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'windowsEventLogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_firewall_logs: {
                        let field_value = match fields_map.get("windowsFirewallLogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'windowsFirewallLogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
