#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDataFlow {
    /// The built-in transform to transform stream data.
    #[builder(into)]
    #[serde(rename = "builtInTransform")]
    pub r#built_in_transform: Option<String>,
    /// Specifies a list of destination names. A `azure_monitor_metrics` data source only allows for stream of kind `Microsoft-InsightsMetrics`.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Vec<String>,
    /// The output stream of the transform. Only required if the data flow changes data to a different stream.
    #[builder(into)]
    #[serde(rename = "outputStream")]
    pub r#output_stream: Option<String>,
    /// Specifies a list of streams. Possible values include but not limited to `Microsoft-Event`, `Microsoft-InsightsMetrics`, `Microsoft-Perf`, `Microsoft-Syslog`, `Microsoft-WindowsEvent`, and `Microsoft-PrometheusMetrics`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Vec<String>,
    /// The KQL query to transform stream data.
    #[builder(into)]
    #[serde(rename = "transformKql")]
    pub r#transform_kql: Option<String>,
}
