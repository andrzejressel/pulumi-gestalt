#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterMaintenancePolicyMaintenanceExclusion {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<String>,
    #[builder(into)]
    #[serde(rename = "exclusionName")]
    pub r#exclusion_name: Box<String>,
    /// Maintenance exclusion related options.
    #[builder(into)]
    #[serde(rename = "exclusionOptions")]
    pub r#exclusion_options: Box<Vec<super::super::types::container::GetClusterMaintenancePolicyMaintenanceExclusionExclusionOption>>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
