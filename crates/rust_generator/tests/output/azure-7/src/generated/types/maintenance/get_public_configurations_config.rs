#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPublicConfigurationsConfig {
    /// A description of the Public Maintenance Configuration.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The duration of the Public Maintenance Configuration window.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// The id of the Public Maintenance Configuration.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The Azure location to filter the list of Public Maintenance Configurations against.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The scope of the Public Maintenance Configuration.
    #[builder(into)]
    #[serde(rename = "maintenanceScope")]
    pub r#maintenance_scope: String,
    /// The name of the Public Maintenance Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The recurring window to filter the list of Public Maintenance Configurations against. Possible values are `Monday-Thursday` and `Friday-Sunday`
    #[builder(into)]
    #[serde(rename = "recurEvery")]
    pub r#recur_every: String,
    /// The time zone for the maintenance window.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
}
