#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IndexingConfigurationThingIndexingConfigurationFilter {
    /// List of shadow names that you select to index.
    #[builder(into, default)]
    #[serde(rename = "namedShadowNames")]
    pub r#named_shadow_names: Box<Option<Vec<String>>>,
}
