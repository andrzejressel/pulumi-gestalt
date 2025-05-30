#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetastoreServiceScalingConfigAutoscalingConfig {
    /// Defines whether autoscaling is enabled. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "autoscalingEnabled")]
    pub r#autoscaling_enabled: Box<Option<bool>>,
    /// Represents the limit configuration of a metastore service.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "limitConfig")]
    pub r#limit_config: Box<Option<super::super::types::dataproc::MetastoreServiceScalingConfigAutoscalingConfigLimitConfig>>,
}
