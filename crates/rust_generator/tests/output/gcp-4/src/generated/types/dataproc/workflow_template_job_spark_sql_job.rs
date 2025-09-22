#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplateJobSparkSqlJob {
    /// HCFS URIs of jar files to be added to the Spark CLASSPATH.
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// The runtime log config for job execution.
    #[builder(into)]
    #[serde(rename = "loggingConfig")]
    pub r#logging_config: Option<Box<super::super::types::dataproc::WorkflowTemplateJobSparkSqlJobLoggingConfig>>,
    /// A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Dataproc API may be overwritten.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// The HCFS URI of the script that contains SQL queries.
    #[builder(into)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Option<String>,
    /// A list of queries.
    #[builder(into)]
    #[serde(rename = "queryList")]
    pub r#query_list: Option<Box<super::super::types::dataproc::WorkflowTemplateJobSparkSqlJobQueryList>>,
    /// Mapping of query variable names to values (equivalent to the Spark SQL command: SET `name="value";`).
    #[builder(into)]
    #[serde(rename = "scriptVariables")]
    pub r#script_variables: Option<std::collections::HashMap<String, String>>,
}
