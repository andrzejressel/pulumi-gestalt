#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkersScriptWebassemblyBinding {
    /// The base64 encoded wasm module you want to store.
    #[builder(into)]
    #[serde(rename = "module")]
    pub r#module: String,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
