#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UptimeCheckConfigSyntheticMonitor {
    /// Target a Synthetic Monitor GCFv2 Instance
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_cloud_function_v2"></a>The `cloud_function_v2` block supports:
    #[builder(into)]
    #[serde(rename = "cloudFunctionV2")]
    pub r#cloud_function_v_2: Box<super::super::types::monitoring::UptimeCheckConfigSyntheticMonitorCloudFunctionV2>,
}
