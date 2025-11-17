#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexingConfigurationThingGroupIndexingConfigurationManagedField {
    /// The name of the field.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The data type of the field. Valid values: `Number`, `String`, `Boolean`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
