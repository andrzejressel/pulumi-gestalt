#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetProjectProject {
    /// Creation time in RFC3339 UTC "Zulu" format.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// A set of key/value label pairs assigned on a project.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The Project lifecycle state.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: String,
    /// The optional user-assigned display name of the project.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The numeric identifier of the project.
    #[builder(into)]
    #[serde(rename = "number")]
    pub r#number: String,
    /// An optional reference to a parent resource.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: std::collections::HashMap<String, String>,
    /// The project id of the project.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: String,
}
