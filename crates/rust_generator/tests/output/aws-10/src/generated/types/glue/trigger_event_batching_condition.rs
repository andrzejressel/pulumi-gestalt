#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerEventBatchingCondition {
    /// Number of events that must be received from Amazon EventBridge before EventBridge  event trigger fires.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: i32,
    /// Window of time in seconds after which EventBridge event trigger fires. Window starts when first event is received. Default value is `900`.
    #[builder(into)]
    #[serde(rename = "batchWindow")]
    pub r#batch_window: Option<i32>,
}
