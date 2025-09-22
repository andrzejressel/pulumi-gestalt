#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionSecretEnvironmentVariable {
    /// Name of the environment variable.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Project identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
    /// ID of the secret in secret manager (not the full resource name).
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: String,
    /// Version of the secret (version number or the string "latest"). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new clones start.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
