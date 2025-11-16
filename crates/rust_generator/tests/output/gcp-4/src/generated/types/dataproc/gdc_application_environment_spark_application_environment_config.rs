#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GdcApplicationEnvironmentSparkApplicationEnvironmentConfig {
    /// A map of default Spark properties to apply to workloads in this application environment. These defaults may be overridden by per-application properties.
    #[builder(into)]
    #[serde(rename = "defaultProperties")]
    pub r#default_properties: Option<std::collections::HashMap<String, String>>,
    /// The default Dataproc version to use for applications submitted to this application environment
    #[builder(into)]
    #[serde(rename = "defaultVersion")]
    pub r#default_version: Option<String>,
}
