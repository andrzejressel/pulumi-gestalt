#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration {
    /// The classification within a configuration.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: String,
    /// A list of additional configurations to apply within a configuration object.
    #[builder(into)]
    #[serde(rename = "configurations")]
    pub r#configurations: Option<Vec<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesApplicationConfigurationConfiguration>>,
    /// A set of properties specified within a configuration classification.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}
