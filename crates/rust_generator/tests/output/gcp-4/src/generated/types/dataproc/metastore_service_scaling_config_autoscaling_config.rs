#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetastoreServiceScalingConfigAutoscalingConfig {
    /// Defines whether autoscaling is enabled. The default value is false.
    #[builder(into)]
    #[serde(rename = "autoscalingEnabled")]
    pub r#autoscaling_enabled: Option<bool>,
    /// Represents the limit configuration of a metastore service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "limitConfig")]
    pub r#limit_config: Option<Box<super::super::types::dataproc::MetastoreServiceScalingConfigAutoscalingConfigLimitConfig>>,
}
