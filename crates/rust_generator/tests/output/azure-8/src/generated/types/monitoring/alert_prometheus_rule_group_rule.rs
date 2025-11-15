#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPrometheusRuleGroupRule {
    /// An `action` block as defined below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRuleAction>>,
    /// Specifies the Alert rule name.
    #[builder(into)]
    #[serde(rename = "alert")]
    pub r#alert: Option<String>,
    /// An `alert_resolution` block as defined below.
    #[builder(into)]
    #[serde(rename = "alertResolution")]
    pub r#alert_resolution: Option<Box<super::super::types::monitoring::AlertPrometheusRuleGroupRuleAlertResolution>>,
    /// Specifies a set of informational labels that can be used to store longer additional information such as alert descriptions or runbook links.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Option<std::collections::HashMap<String, String>>,
    /// Is this rule enabled? Possible values are `true` and `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Specifies the Prometheus Query Language expression to evaluate. For more details see [this doc](https://prometheus.io/docs/prometheus/latest/querying/basics). Evaluate at the period given by `interval` and record the result as a new set of time series with the metric name given by `record`.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: String,
    /// Specifies the amount of time alert must be active before firing, represented in ISO 8601 duration format.
    #[builder(into)]
    #[serde(rename = "for")]
    pub r#for_: Option<String>,
    /// Specifies the labels to add or overwrite before storing the result.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Specifies the recorded metrics name.
    #[builder(into)]
    #[serde(rename = "record")]
    pub r#record: Option<String>,
    /// Specifies the severity of the alerts fired by the rule. Possible values are between 0 and 4.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<i32>,
}
