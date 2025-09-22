#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoggingConfigurationLoggingConfiguration {
    /// Set of configuration blocks describing the logging details for a firewall. See Log Destination Config below for details. At most, only Three blocks can be specified; one for `FLOW` logs and one for `ALERT` logs and one for `TLS` logs.
    #[builder(into)]
    #[serde(rename = "logDestinationConfigs")]
    pub r#log_destination_configs: Vec<super::super::types::networkfirewall::LoggingConfigurationLoggingConfigurationLogDestinationConfig>,
}
