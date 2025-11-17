#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionCloudSql {
    /// Cloud SQL properties.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "credential")]
    pub r#credential: Box<super::super::types::bigquery::ConnectionCloudSqlCredential>,
    /// Database name.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// Cloud SQL instance ID in the form project:location:instance.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: String,
    /// (Output)
    /// When the connection is used in the context of an operation in BigQuery, this service account will serve as the identity being used for connecting to the CloudSQL instance specified in this connection.
    #[builder(into)]
    #[serde(rename = "serviceAccountId")]
    pub r#service_account_id: Option<String>,
    /// Type of the Cloud SQL database.
    /// Possible values are: `DATABASE_TYPE_UNSPECIFIED`, `POSTGRES`, `MYSQL`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
