#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobSparksqlConfig {
    /// HCFS URIs of jar files to be added to the Spark CLASSPATH.
    /// 
    /// * `logging_config.driver_log_levels`- (Required) The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// The runtime logging config of the job
    #[builder(into)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Option<Box<super::super::types::dataproc::JobSparksqlConfigLoggingConfig>>,
    /// A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    /// Conflicts with `query_list`
    #[builder(into)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Option<String>,
    /// The list of SQL queries or statements to execute as part of the job.
    /// Conflicts with `query_file_uri`
    #[builder(into)]
    #[serde(rename = "queryLists")]
    pub r#query_lists: Option<Vec<String>>,
    /// Mapping of query variable names to values (equivalent to the Spark SQL command: `SET name="value";`).
    #[builder(into)]
    #[serde(rename = "scriptVariables")]
    pub r#script_variables: Option<std::collections::HashMap<String, String>>,
}
