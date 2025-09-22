#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
    #[builder(into)]
    #[serde(rename = "autoDeployConfidenceThreshold")]
    pub r#auto_deploy_confidence_threshold: f64,
    #[builder(into)]
    #[serde(rename = "autoDeployExpirationSec")]
    pub r#auto_deploy_expiration_sec: i32,
    #[builder(into)]
    #[serde(rename = "autoDeployImpactedBaselineThreshold")]
    pub r#auto_deploy_impacted_baseline_threshold: f64,
    #[builder(into)]
    #[serde(rename = "autoDeployLoadThreshold")]
    pub r#auto_deploy_load_threshold: f64,
    #[builder(into)]
    #[serde(rename = "detectionAbsoluteQps")]
    pub r#detection_absolute_qps: f64,
    #[builder(into)]
    #[serde(rename = "detectionLoadThreshold")]
    pub r#detection_load_threshold: f64,
    #[builder(into)]
    #[serde(rename = "detectionRelativeToBaselineQps")]
    pub r#detection_relative_to_baseline_qps: f64,
    /// The name of the security policy. Provide either this or a `self_link`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "trafficGranularityConfigs")]
    pub r#traffic_granularity_configs: Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig>,
}
