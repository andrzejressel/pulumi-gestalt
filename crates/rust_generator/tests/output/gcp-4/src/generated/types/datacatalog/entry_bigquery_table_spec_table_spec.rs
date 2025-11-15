#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EntryBigqueryTableSpecTableSpec {
    /// (Output)
    /// If the table is a dated shard, i.e., with name pattern [prefix]YYYYMMDD, groupedEntry is the
    /// Data Catalog resource name of the date sharded grouped entry, for example,
    /// projects/{project_id}/locations/{location}/entrygroups/{entryGroupId}/entries/{entryId}.
    /// Otherwise, groupedEntry is empty.
    #[builder(into)]
    #[serde(rename = "groupedEntry")]
    pub r#grouped_entry: Option<String>,
}
