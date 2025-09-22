#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateTemplateContainerEnvValueSourceSecretKeyRef {
    /// The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: String,
    /// The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
