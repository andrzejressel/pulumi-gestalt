#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileSqlServerProfile {
    /// Database for the SQL Server connection.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Hostname for the SQL Server connection.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// Password for the SQL Server connection.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Port for the SQL Server connection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Username for the SQL Server connection.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
