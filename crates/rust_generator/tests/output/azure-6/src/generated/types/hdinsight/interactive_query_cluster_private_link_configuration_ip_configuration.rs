#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InteractiveQueryClusterPrivateLinkConfigurationIpConfiguration {
    /// Specifies the name for this HDInsight Interactive Query Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates whether this IP configuration is primary.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// The private IP address of the IP configuration.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Option<String>,
    /// The private IP allocation method. The only possible value now is `Dynamic`.
    #[builder(into)]
    #[serde(rename = "privateIpAllocationMethod")]
    pub r#private_ip_allocation_method: Option<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
