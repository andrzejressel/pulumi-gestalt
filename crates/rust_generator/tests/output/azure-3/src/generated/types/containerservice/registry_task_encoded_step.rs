#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistryTaskEncodedStep {
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: Option<String>,
    /// The URL (absolute or relative) of the source context for this step.
    #[builder(into)]
    #[serde(rename = "contextPath")]
    pub r#context_path: Option<String>,
    /// Specifies a map of secret values that can be passed when running a task.
    #[builder(into)]
    #[serde(rename = "secretValues")]
    pub r#secret_values: Option<std::collections::HashMap<String, String>>,
    /// The (optionally base64 encoded) content of the build template.
    #[builder(into)]
    #[serde(rename = "taskContent")]
    pub r#task_content: String,
    /// The (optionally base64 encoded) content of the build parameters.
    #[builder(into)]
    #[serde(rename = "valueContent")]
    pub r#value_content: Option<String>,
    /// Specifies a map of values that can be passed when running a task.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<std::collections::HashMap<String, String>>,
}
