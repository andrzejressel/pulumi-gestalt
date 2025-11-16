#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GameServerGroupLaunchTemplate {
    /// A unique identifier for an existing EC2 launch template.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// A readable identifier for an existing EC2 launch template.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The version of the EC2 launch template to use. If none is set, the default is the first version created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
