#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfigHandlerMapping {
    /// Specify the command-line arguments to be passed to the script processor.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: Option<String>,
    /// Specify which extension to be handled by the specified FastCGI application.
    #[builder(into)]
    #[serde(rename = "extension")]
    pub r#extension: String,
    /// Specify the absolute path to the FastCGI application.
    #[builder(into)]
    #[serde(rename = "scriptProcessorPath")]
    pub r#script_processor_path: String,
}
