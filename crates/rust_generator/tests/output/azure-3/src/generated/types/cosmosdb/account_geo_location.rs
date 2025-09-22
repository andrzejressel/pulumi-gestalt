#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountGeoLocation {
    /// The failover priority of the region. A failover priority of `0` indicates a write region. The maximum value for a failover priority = (total number of regions - 1). Failover priority values must be unique for each of the regions in which the database account exists. Changing this causes the location to be re-provisioned and cannot be changed for the location with failover priority `0`.
    #[builder(into)]
    #[serde(rename = "failoverPriority")]
    pub r#failover_priority: i32,
    /// The CosmosDB Account ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the Azure region to host replicated data.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// Should zone redundancy be enabled for this region? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "zoneRedundant")]
    pub r#zone_redundant: Option<bool>,
}
