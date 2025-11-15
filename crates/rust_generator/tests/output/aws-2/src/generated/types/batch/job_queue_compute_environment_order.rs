#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobQueueComputeEnvironmentOrder {
    /// The Amazon Resource Name (ARN) of the compute environment.
    #[builder(into)]
    #[serde(rename = "computeEnvironment")]
    pub r#compute_environment: String,
    /// The order of the compute environment. Compute environments are tried in ascending order. For example, if two compute environments are associated with a job queue, the compute environment with a lower order integer value is tried for job placement first.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
}
