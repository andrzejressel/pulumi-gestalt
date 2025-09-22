#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceDataSourceBackupApplianceApplication {
    /// Appliance Id of the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applianceId")]
    pub r#appliance_id: String,
    /// The appid field of the application within the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: String,
    /// The name of the Application as known to the Backup Appliance.
    #[builder(into)]
    #[serde(rename = "applicationName")]
    pub r#application_name: String,
    /// Appliance name.
    #[builder(into)]
    #[serde(rename = "backupAppliance")]
    pub r#backup_appliance: String,
    /// Hostid of the application host.
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: String,
    /// Hostname of the host where the application is running.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// The type of the application. e.g. VMBackup
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
