#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationStoreReplica {
    /// The URL of the App Configuration Replica.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
    /// The ID of the Access Key.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the supported Azure location where the replica exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// Specifies the name of the replica.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
