#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ModuleModuleLink {
    /// A `hash` block as defined below.
    #[builder(into)]
    #[serde(rename = "hash")]
    pub r#hash: Option<Box<super::super::types::automation::ModuleModuleLinkHash>>,
    /// The URI of the module content (zip or nupkg).
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
