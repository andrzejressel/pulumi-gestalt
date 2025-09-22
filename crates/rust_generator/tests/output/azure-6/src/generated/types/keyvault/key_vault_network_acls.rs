#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeyVaultNetworkAcls {
    /// Specifies which traffic can bypass the network rules. Possible values are `AzureServices` and `None`.
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: String,
    /// The Default Action to use when no rules match from `ip_rules` / `virtual_network_subnet_ids`. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// One or more IP Addresses, or CIDR Blocks which should be able to access the Key Vault.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<String>>,
    /// One or more Subnet IDs which should be able to access this Key Vault.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Option<Vec<String>>,
}
