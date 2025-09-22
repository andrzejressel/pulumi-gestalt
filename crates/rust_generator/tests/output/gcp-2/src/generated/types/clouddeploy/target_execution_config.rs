#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetExecutionConfig {
    /// Optional. Cloud Storage location in which to store execution outputs. This can either be a bucket ("gs://my-bucket") or a path within a bucket ("gs://my-bucket/my-dir"). If unspecified, a default bucket located in the same region will be used.
    #[builder(into)]
    #[serde(rename = "artifactStorage")]
    pub r#artifact_storage: Option<String>,
    /// Optional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used.
    #[builder(into)]
    #[serde(rename = "executionTimeout")]
    pub r#execution_timeout: Option<String>,
    /// Optional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Required. Usages when this configuration should be applied.
    #[builder(into)]
    #[serde(rename = "usages")]
    pub r#usages: Vec<String>,
    /// Optional. If true, additional logging will be enabled when running builds in this execution environment.
    #[builder(into)]
    #[serde(rename = "verbose")]
    pub r#verbose: Option<bool>,
    /// Optional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Option<String>,
}
