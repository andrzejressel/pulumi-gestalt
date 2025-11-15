#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CollectorPolicyIpfxIngestion {
    /// A list of ingestion source resource IDs. Changing this forces a new Network Function Collector Policy to be created.
    #[builder(into)]
    #[serde(rename = "sourceResourceIds")]
    pub r#source_resource_ids: Vec<String>,
}
