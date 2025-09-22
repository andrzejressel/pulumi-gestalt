#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfilePostgresql {
    /// If the connected database is an AlloyDB instance, use this field to provide the AlloyDB cluster ID.
    #[builder(into)]
    #[serde(rename = "alloydbClusterId")]
    pub r#alloydb_cluster_id: Option<String>,
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[builder(into)]
    #[serde(rename = "cloudSqlId")]
    pub r#cloud_sql_id: Option<String>,
    /// The IP or hostname of the source MySQL database.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// (Output)
    /// Output only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with.
    #[builder(into)]
    #[serde(rename = "networkArchitecture")]
    pub r#network_architecture: Option<String>,
    /// Input only. The password for the user that Database Migration Service will be using to connect to the database.
    /// This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// (Output)
    /// Output only. Indicates If this connection profile password is stored.
    #[builder(into)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Option<bool>,
    /// The network port of the source MySQL database.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ssl")]
    pub r#ssl: Option<Box<super::super::types::databasemigrationservice::ConnectionProfilePostgresqlSsl>>,
    /// The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
