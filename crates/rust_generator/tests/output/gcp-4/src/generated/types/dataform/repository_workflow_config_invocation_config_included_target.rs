#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryWorkflowConfigInvocationConfigIncludedTarget {
    /// The action's database (Google Cloud project ID).
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Option<String>,
    /// The action's name, within database and schema.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The action's schema (BigQuery dataset ID), within database.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
}
