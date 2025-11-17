#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventTargetRedshiftTarget {
    /// The name of the database.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// The database user name.
    #[builder(into)]
    #[serde(rename = "dbUser")]
    pub r#db_user: Option<String>,
    /// The name or ARN of the secret that enables access to the database.
    #[builder(into)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: Option<String>,
    /// The SQL statement text to run.
    #[builder(into)]
    #[serde(rename = "sql")]
    pub r#sql: Option<String>,
    /// The name of the SQL statement.
    #[builder(into)]
    #[serde(rename = "statementName")]
    pub r#statement_name: Option<String>,
    /// Indicates whether to send an event back to EventBridge after the SQL statement runs.
    #[builder(into)]
    #[serde(rename = "withEvent")]
    pub r#with_event: Option<bool>,
}
