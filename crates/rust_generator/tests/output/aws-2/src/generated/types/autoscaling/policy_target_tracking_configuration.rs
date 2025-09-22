#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyTargetTrackingConfiguration {
    /// Customized metric. Conflicts with `predefined_metric_specification`.
    #[builder(into)]
    #[serde(rename = "customizedMetricSpecification")]
    pub r#customized_metric_specification: Option<Box<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecification>>,
    /// Whether scale in by the target tracking policy is disabled.
    #[builder(into)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Option<bool>,
    /// Predefined metric. Conflicts with `customized_metric_specification`.
    #[builder(into)]
    #[serde(rename = "predefinedMetricSpecification")]
    pub r#predefined_metric_specification: Option<Box<super::super::types::autoscaling::PolicyTargetTrackingConfigurationPredefinedMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: f64,
}
