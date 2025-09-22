#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupLaunchTemplate {
    /// ID of the launch template.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Specify the exact name of the desired autoscaling group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Template version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
