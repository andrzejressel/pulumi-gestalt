#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskDefinitionVolumeDockerVolumeConfiguration {
    /// If this value is `true`, the Docker volume is created if it does not already exist. *Note*: This field is only used if the scope is `shared`.
    #[builder(into)]
    #[serde(rename = "autoprovision")]
    pub r#autoprovision: Option<bool>,
    /// Docker volume driver to use. The driver value must match the driver name provided by Docker because it is used for task placement.
    #[builder(into)]
    #[serde(rename = "driver")]
    pub r#driver: Option<String>,
    /// Map of Docker driver specific options.
    #[builder(into)]
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Option<std::collections::HashMap<String, String>>,
    /// Map of custom metadata to add to your Docker volume.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Scope for the Docker volume, which determines its lifecycle, either `task` or `shared`.  Docker volumes that are scoped to a `task` are automatically provisioned when the task starts and destroyed when the task stops. Docker volumes that are scoped as `shared` persist after the task stops.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
}
