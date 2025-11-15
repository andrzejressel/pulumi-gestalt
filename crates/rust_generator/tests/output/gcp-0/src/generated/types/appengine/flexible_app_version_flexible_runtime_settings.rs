#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionFlexibleRuntimeSettings {
    /// Operating System of the application runtime.
    #[builder(into)]
    #[serde(rename = "operatingSystem")]
    pub r#operating_system: Option<String>,
    /// The runtime version of an App Engine flexible application.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Option<String>,
}
