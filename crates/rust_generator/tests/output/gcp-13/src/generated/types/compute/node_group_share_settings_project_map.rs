#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeGroupShareSettingsProjectMap {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The project id/number should be the same as the key of this project config in the project map.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
}
