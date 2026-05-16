#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReservationShareSetting {
    /// A map of project number and project config. This is only valid when shareType's value is SPECIFIC_PROJECTS.
    #[builder(into)]
    #[serde(rename = "projectMaps")]
    pub r#project_maps: Vec<super::super::types::compute::GetReservationShareSettingProjectMap>,
    /// Type of sharing for this shared-reservation Possible values: ["LOCAL", "SPECIFIC_PROJECTS"]
    #[builder(into)]
    #[serde(rename = "shareType")]
    pub r#share_type: String,
}
