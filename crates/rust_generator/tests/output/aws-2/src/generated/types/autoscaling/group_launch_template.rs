#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupLaunchTemplate {
    /// ID of the launch template. Conflicts with `name`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Name of the launch template. Conflicts with `id`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Template version. Can be version number, `$Latest`, or `$Default`. (Default: `$Default`).
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
