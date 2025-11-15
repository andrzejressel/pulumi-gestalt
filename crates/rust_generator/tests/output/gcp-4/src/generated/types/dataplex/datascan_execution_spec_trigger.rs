#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanExecutionSpecTrigger {
    /// The scan runs once via dataScans.run API.
    #[builder(into)]
    #[serde(rename = "onDemand")]
    pub r#on_demand: Option<Box<super::super::types::dataplex::DatascanExecutionSpecTriggerOnDemand>>,
    /// The scan is scheduled to run periodically.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<Box<super::super::types::dataplex::DatascanExecutionSpecTriggerSchedule>>,
}
