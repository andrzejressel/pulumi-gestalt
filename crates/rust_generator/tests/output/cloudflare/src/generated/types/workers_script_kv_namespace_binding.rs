#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkersScriptKvNamespaceBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// ID of the KV namespace you want to use.
    #[builder(into)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: String,
}
