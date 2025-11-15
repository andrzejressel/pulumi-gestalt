#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTablePartitionKey {
    /// Free-form text comment.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// Name of the Partition Key.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Datatype of data in the Partition Key.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
