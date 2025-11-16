#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigMonitoring {
    /// Configuration for logging requests made to this project to Stackdriver Logging
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestLogging")]
    pub r#request_logging: Option<Box<super::super::types::identityplatform::ConfigMonitoringRequestLogging>>,
}
