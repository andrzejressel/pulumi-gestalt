#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVolumeGroupNetworkRule {
    /// The action to take when an access attempt to this Elastic SAN Volume Group from this Subnet is made.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The ID of the Subnet from which access to this Elastic SAN Volume Group is allowed.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
