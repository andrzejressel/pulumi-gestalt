#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowTriggerConfigTriggerProperties {
    #[builder(into)]
    #[serde(rename = "scheduled")]
    pub r#scheduled: Box<Option<super::super::types::appflow::FlowTriggerConfigTriggerPropertiesScheduled>>,
}
