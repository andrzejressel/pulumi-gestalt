#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersRedshiftDataParameters {
    /// The name of the database. Required when authenticating using temporary credentials.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// The database user name. Required when authenticating using temporary credentials.
    #[builder(into)]
    #[serde(rename = "dbUser")]
    pub r#db_user: Option<String>,
    /// The name or ARN of the secret that enables access to the database. Required when authenticating using Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretManagerArn")]
    pub r#secret_manager_arn: Option<String>,
    /// List of SQL statements text to run, each of maximum length of 100,000.
    #[builder(into)]
    #[serde(rename = "sqls")]
    pub r#sqls: Vec<String>,
    /// The name of the SQL statement. You can name the SQL statement when you create it to identify the query.
    #[builder(into)]
    #[serde(rename = "statementName")]
    pub r#statement_name: Option<String>,
    /// Indicates whether to send an event back to EventBridge after the SQL statement runs.
    #[builder(into)]
    #[serde(rename = "withEvent")]
    pub r#with_event: Option<bool>,
}
