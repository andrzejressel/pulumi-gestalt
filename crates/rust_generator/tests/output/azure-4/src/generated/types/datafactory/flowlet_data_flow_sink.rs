#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowletDataFlowSink {
    /// A `dataset` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Option<Box<super::super::types::datafactory::FlowletDataFlowSinkDataset>>,
    /// The description for the Data Flow Source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A `flowlet` block as defined below.
    #[builder(into)]
    #[serde(rename = "flowlet")]
    pub r#flowlet: Option<Box<super::super::types::datafactory::FlowletDataFlowSinkFlowlet>>,
    /// A `linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "linkedService")]
    pub r#linked_service: Option<Box<super::super::types::datafactory::FlowletDataFlowSinkLinkedService>>,
    /// The name for the Data Flow Source.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `rejected_linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "rejectedLinkedService")]
    pub r#rejected_linked_service: Option<Box<super::super::types::datafactory::FlowletDataFlowSinkRejectedLinkedService>>,
    /// A `schema_linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "schemaLinkedService")]
    pub r#schema_linked_service: Option<Box<super::super::types::datafactory::FlowletDataFlowSinkSchemaLinkedService>>,
}
