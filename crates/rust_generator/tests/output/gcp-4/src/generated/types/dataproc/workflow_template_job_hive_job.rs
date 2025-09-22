#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplateJobHiveJob {
    /// Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries.
    #[builder(into)]
    #[serde(rename = "continueOnFailure")]
    pub r#continue_on_failure: Option<bool>,
    /// HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs.
    #[builder(into)]
    #[serde(rename = "jarFileUris")]
    pub r#jar_file_uris: Option<Vec<String>>,
    /// A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// The HCFS URI of the script that contains Hive queries.
    #[builder(into)]
    #[serde(rename = "queryFileUri")]
    pub r#query_file_uri: Option<String>,
    /// A list of queries.
    #[builder(into)]
    #[serde(rename = "queryList")]
    pub r#query_list: Option<Box<super::super::types::dataproc::WorkflowTemplateJobHiveJobQueryList>>,
    /// Mapping of query variable names to values (equivalent to the Hive command: `SET name="value";`).
    #[builder(into)]
    #[serde(rename = "scriptVariables")]
    pub r#script_variables: Option<std::collections::HashMap<String, String>>,
}
