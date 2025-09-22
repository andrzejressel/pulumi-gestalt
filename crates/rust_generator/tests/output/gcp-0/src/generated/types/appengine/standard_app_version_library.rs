#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StandardAppVersionLibrary {
    /// Name of the library. Example "django".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Version of the library to select, or "latest".
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
