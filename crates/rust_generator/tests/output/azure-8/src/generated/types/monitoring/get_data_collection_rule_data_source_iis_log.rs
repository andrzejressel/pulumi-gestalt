#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataCollectionRuleDataSourceIisLog {
    /// Specifies a list of absolute paths where the log files are located.
    #[builder(into)]
    #[serde(rename = "logDirectories")]
    pub r#log_directories: Vec<String>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Vec<String>,
}
