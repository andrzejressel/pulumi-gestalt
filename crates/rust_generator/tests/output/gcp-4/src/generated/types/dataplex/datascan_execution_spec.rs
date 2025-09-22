#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatascanExecutionSpec {
    /// The unnested field (of type Date or Timestamp) that contains values which monotonically increase over time. If not specified, a data scan will run for all data in the table.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    /// Spec related to how often and when a scan should be triggered.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<super::super::types::dataplex::DatascanExecutionSpecTrigger>,
}
