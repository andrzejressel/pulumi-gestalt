#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSalesforce {
    /// Flag that enables dynamic fetching of new (recently added) fields in the Salesforce objects while running a flow.
    #[builder(into)]
    #[serde(rename = "enableDynamicFieldUpdate")]
    pub r#enable_dynamic_field_update: Option<bool>,
    /// Whether Amazon AppFlow includes deleted files in the flow run.
    #[builder(into)]
    #[serde(rename = "includeDeletedRecords")]
    pub r#include_deleted_records: Option<bool>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}
