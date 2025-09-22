#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFrameworkControlSet {
    #[builder(into)]
    #[serde(rename = "controls")]
    pub r#controls: Option<Vec<super::super::types::auditmanager::GetFrameworkControlSetControl>>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Name of the framework.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
