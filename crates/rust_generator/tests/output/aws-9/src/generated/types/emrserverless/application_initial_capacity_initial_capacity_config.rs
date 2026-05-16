#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationInitialCapacityInitialCapacityConfig {
    /// The resource configuration of the initial capacity configuration.
    #[builder(into)]
    #[serde(rename = "workerConfiguration")]
    pub r#worker_configuration: Option<Box<super::super::types::emrserverless::ApplicationInitialCapacityInitialCapacityConfigWorkerConfiguration>>,
    /// The number of workers in the initial capacity configuration.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: i32,
}
