#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AzureClusterLoggingConfigComponentConfig {
    /// Components of the logging configuration to be enabled.
    #[builder(into, default)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Box<Option<Vec<String>>>,
}
