#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPrivateDnsZoneConfig {
    /// The ID of the Private DNS Zone Config.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the Name of the Private Endpoint. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A list of IP Addresses
    #[builder(into)]
    #[serde(rename = "privateDnsZoneId")]
    pub r#private_dns_zone_id: Option<String>,
    /// A `record_sets` block as defined below.
    #[builder(into)]
    #[serde(rename = "recordSets")]
    pub r#record_sets: Option<Vec<super::super::types::privatelink::EndpointPrivateDnsZoneConfigRecordSet>>,
}
