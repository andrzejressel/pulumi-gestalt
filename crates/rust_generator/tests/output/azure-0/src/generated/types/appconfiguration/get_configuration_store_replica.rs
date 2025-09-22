#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationStoreReplica {
    /// The URL of the App Configuration Replica.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: String,
    /// The ID of the Access Key.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The supported Azure location where the App Configuration Replica exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The Name of this App Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
