#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodeGroupShareSettings {
    /// A map of project id and project config. This is only valid when shareType's value is SPECIFIC_PROJECTS.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "projectMaps")]
    pub r#project_maps: Option<Vec<super::super::types::compute::NodeGroupShareSettingsProjectMap>>,
    /// Node group sharing type.
    /// Possible values are: `ORGANIZATION`, `SPECIFIC_PROJECTS`, `LOCAL`.
    #[builder(into)]
    #[serde(rename = "shareType")]
    pub r#share_type: String,
}
