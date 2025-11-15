#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsqlSettingsIpConfig {
    /// The list of external networks that are allowed to connect to the instance using the IP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Option<Vec<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfigAuthorizedNetwork>>,
    /// Whether the instance should be assigned an IPv4 address or not.
    #[builder(into)]
    #[serde(rename = "enableIpv4")]
    pub r#enable_ipv_4: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.
    /// This setting can be updated, but it cannot be removed after it is set.
    #[builder(into)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: Option<String>,
    /// Whether SSL connections over IP should be enforced or not.
    #[builder(into)]
    #[serde(rename = "requireSsl")]
    pub r#require_ssl: Option<bool>,
}
