#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration {
    /// The log driver to use for the container.
    #[builder(into)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: String,
    /// The configuration options to send to the log driver.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: std::collections::HashMap<String, String>,
    /// The secrets to pass to the log configuration.
    #[builder(into)]
    #[serde(rename = "secretOptions")]
    pub r#secret_options: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfigurationSecretOption>,
}
