#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskExcludes {
    /// The type of filter rule to apply. Valid values: `SIMPLE_PATTERN`.
    #[builder(into)]
    #[serde(rename = "filterType")]
    pub r#filter_type: Option<String>,
    /// A single filter string that consists of the patterns to exclude. The patterns are delimited by "|" (that is, a pipe), for example: `/folder1|/folder2`
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
