#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceRelationalDatabaseConfigHttpEndpointConfig {
    /// AWS secret store ARN for database credentials.
    #[builder(into)]
    #[serde(rename = "awsSecretStoreArn")]
    pub r#aws_secret_store_arn: String,
    /// Logical database name.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Option<String>,
    /// Amazon RDS cluster identifier.
    #[builder(into)]
    #[serde(rename = "dbClusterIdentifier")]
    pub r#db_cluster_identifier: String,
    /// AWS Region for RDS HTTP endpoint. Defaults to current region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Logical schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
}
