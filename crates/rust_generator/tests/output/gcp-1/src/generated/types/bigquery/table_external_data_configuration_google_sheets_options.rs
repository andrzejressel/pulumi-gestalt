#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfigurationGoogleSheetsOptions {
    /// Range of a sheet to query from. Only used when
    /// non-empty. At least one of `range` or `skip_leading_rows` must be set.
    /// Typical format: "sheet_name!top_left_cell_id:bottom_right_cell_id"
    /// For example: "sheet1!A1:B20"
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Option<String>,
    /// The number of rows at the top of the sheet
    /// that BigQuery will skip when reading the data. At least one of `range` or
    /// `skip_leading_rows` must be set.
    #[builder(into)]
    #[serde(rename = "skipLeadingRows")]
    pub r#skip_leading_rows: Option<i32>,
}
