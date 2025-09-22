#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetRefreshPropertiesRefreshConfigurationIncrementalRefreshLookbackWindow {
    /// The name of the lookback window column.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: String,
    /// The lookback window column size.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// The size unit that is used for the lookback window column. Valid values for this structure are `HOUR`, `DAY`, and `WEEK`.
    #[builder(into)]
    #[serde(rename = "sizeUnit")]
    pub r#size_unit: String,
}
