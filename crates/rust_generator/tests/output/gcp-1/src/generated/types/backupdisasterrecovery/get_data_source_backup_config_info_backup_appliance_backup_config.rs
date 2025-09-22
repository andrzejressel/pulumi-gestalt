#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceBackupConfigInfoBackupApplianceBackupConfig {
    /// The name of the application.
    #[builder(into)]
    #[serde(rename = "applicationName")]
    pub r#application_name: String,
    /// The ID of the backup appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceId")]
    pub r#backup_appliance_id: String,
    /// The name of the backup appliance.
    #[builder(into)]
    #[serde(rename = "backupApplianceName")]
    pub r#backup_appliance_name: String,
    /// The name of the host where the application is running.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the SLA of this application.
    #[builder(into)]
    #[serde(rename = "slaId")]
    pub r#sla_id: String,
    /// The name of the SLP associated with the application.
    #[builder(into)]
    #[serde(rename = "slpName")]
    pub r#slp_name: String,
    /// The name of the SLT associated with the application.
    #[builder(into)]
    #[serde(rename = "sltName")]
    pub r#slt_name: String,
}
