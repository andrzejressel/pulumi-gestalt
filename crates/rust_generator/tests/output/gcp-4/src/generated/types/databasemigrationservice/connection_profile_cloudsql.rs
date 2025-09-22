#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileCloudsql {
    /// (Output)
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[builder(into)]
    #[serde(rename = "cloudSqlId")]
    pub r#cloud_sql_id: Option<String>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's private IP.
    #[builder(into)]
    #[serde(rename = "privateIp")]
    pub r#private_ip: Option<String>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's public IP.
    #[builder(into)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: Option<String>,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettings>>,
}
