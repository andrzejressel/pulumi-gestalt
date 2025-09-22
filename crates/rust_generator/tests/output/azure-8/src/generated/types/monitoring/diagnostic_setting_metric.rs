#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DiagnosticSettingMetric {
    /// The name of a Diagnostic Metric Category for this Resource.
    /// 
    /// > **NOTE:** The Metric Categories available vary depending on the Resource being used. You may wish to use the `azure.monitoring.getDiagnosticCategories` Data Source to identify which categories are available for a given Resource.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// Is this Diagnostic Metric enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Option<Box<super::super::types::monitoring::DiagnosticSettingMetricRetentionPolicy>>,
}
