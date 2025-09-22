#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightsReportConfigCsvOptions {
    /// The delimiter used to separate the fields in the inventory report CSV file.
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Option<String>,
    /// The boolean that indicates whether or not headers are included in the inventory report CSV file.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "headerRequired")]
    pub r#header_required: Option<bool>,
    /// The character used to separate the records in the inventory report CSV file.
    #[builder(into)]
    #[serde(rename = "recordSeparator")]
    pub r#record_separator: Option<String>,
}
