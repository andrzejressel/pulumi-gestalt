#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetStandardConnectionString {
    /// The name of this Logic App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Type of Managed Identity assigned to this Logic App Workflow.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
