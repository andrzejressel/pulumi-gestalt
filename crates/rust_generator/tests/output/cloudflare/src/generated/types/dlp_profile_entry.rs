#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DlpProfileEntry {
    /// Whether the entry is active. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Unique entry identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Name of the entry to deploy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Option<Box<super::types::DlpProfileEntryPattern>>,
}
