#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineVariable {
    /// The default value of a pipeline-level variable.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// The description of a pipeline-level variable.
    /// 
    /// > **Note:** The input artifact of an action must exactly match the output artifact declared in a preceding action, but the input artifact does not have to be the next action in strict sequence from the action that provided the output artifact. Actions in parallel can declare different output artifacts, which are in turn consumed by different following actions.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The name of a pipeline-level variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
