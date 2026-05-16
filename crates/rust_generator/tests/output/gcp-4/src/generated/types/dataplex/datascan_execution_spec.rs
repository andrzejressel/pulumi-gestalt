#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
