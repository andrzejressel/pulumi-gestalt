#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataCollectionRuleDataSourceLogFile {
    /// Specifies a list of file patterns where the log files are located. For example, `C:\\JavaLogs\\*.log`.
    #[builder(into)]
    #[serde(rename = "filePatterns")]
    pub r#file_patterns: Box<Vec<String>>,
    /// The data format of the log files. possible value is `text`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `settings` block as defined below.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceLogFileSetting>>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
