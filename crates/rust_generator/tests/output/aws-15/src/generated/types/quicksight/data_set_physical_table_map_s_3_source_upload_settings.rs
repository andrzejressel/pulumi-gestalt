#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetPhysicalTableMapS3SourceUploadSettings {
    /// Whether the file has a header row, or the files each have a header row.
    #[builder(into)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: Option<bool>,
    /// Delimiter between values in the file.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Option<String>,
    /// File format. Valid values are `CSV`, `TSV`, `CLF`, `ELF`, `XLSX`, and `JSON`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// A row number to start reading data from.
    #[builder(into)]
    #[serde(rename = "startFromRow")]
    pub r#start_from_row: Option<i32>,
    /// Text qualifier. Valid values are `DOUBLE_QUOTE` and `SINGLE_QUOTE`.
    #[builder(into)]
    #[serde(rename = "textQualifier")]
    pub r#text_qualifier: Option<String>,
}
