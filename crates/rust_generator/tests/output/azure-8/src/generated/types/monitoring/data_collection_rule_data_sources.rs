#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
