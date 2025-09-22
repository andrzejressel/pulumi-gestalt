#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetManagedZonesManagedZone {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: String,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    #[builder(into)]
    #[serde(rename = "managedZoneId")]
    pub r#managed_zone_id: String,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "nameServers")]
    pub r#name_servers: Vec<String>,
    /// The ID of the project containing Google Cloud DNS zones. If this is not provided the default project will be used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Option<String>,
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: String,
}
