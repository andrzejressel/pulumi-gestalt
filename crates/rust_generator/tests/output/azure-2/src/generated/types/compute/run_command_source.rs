#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RunCommandSource {
    #[builder(into)]
    #[serde(rename = "commandId")]
    pub r#command_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<String>,
    #[builder(into)]
    #[serde(rename = "scriptUri")]
    pub r#script_uri: Option<String>,
    /// A `script_uri_managed_identity` block as defined above.
    #[builder(into)]
    #[serde(rename = "scriptUriManagedIdentity")]
    pub r#script_uri_managed_identity: Option<Box<super::super::types::compute::RunCommandSourceScriptUriManagedIdentity>>,
}
