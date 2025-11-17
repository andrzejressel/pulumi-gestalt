#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssetDiscoveryStatusStat {
    /// The count of data items within the referenced resource.
    #[builder(into)]
    #[serde(rename = "dataItems")]
    pub r#data_items: Option<i32>,
    /// The number of stored data bytes within the referenced resource.
    #[builder(into)]
    #[serde(rename = "dataSize")]
    pub r#data_size: Option<i32>,
    /// The count of fileset entities within the referenced resource.
    #[builder(into)]
    #[serde(rename = "filesets")]
    pub r#filesets: Option<i32>,
    /// The count of table entities within the referenced resource.
    #[builder(into)]
    #[serde(rename = "tables")]
    pub r#tables: Option<i32>,
}
