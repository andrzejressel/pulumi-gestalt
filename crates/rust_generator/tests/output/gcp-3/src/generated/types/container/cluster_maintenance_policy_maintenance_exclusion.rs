#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMaintenancePolicyMaintenanceExclusion {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: String,
    #[builder(into)]
    #[serde(rename = "exclusionName")]
    pub r#exclusion_name: String,
    /// MaintenanceExclusionOptions provides maintenance exclusion related options.
    #[builder(into)]
    #[serde(rename = "exclusionOptions")]
    pub r#exclusion_options: Option<Box<super::super::types::container::ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions>>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
}
