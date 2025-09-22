#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NamespaceNetworkRuleSet {
    /// Specifies the default action for the Network Rule Set. Possible values are `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Option<String>,
    /// One or more IP Addresses, or CIDR Blocks which should be able to access the ServiceBus Namespace.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<String>>,
    /// One or more `network_rules` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "networkRules")]
    pub r#network_rules: Option<Vec<super::super::types::eventhub::NamespaceNetworkRuleSetNetworkRule>>,
    /// Whether to allow traffic over public network. Possible values are `true` and `false`. Defaults to `true`.
    /// 
    /// > **Note:** To disable public network access, you must also configure the property `public_network_access_enabled`.
    #[builder(into)]
    #[serde(rename = "publicNetworkAccessEnabled")]
    pub r#public_network_access_enabled: Option<bool>,
    /// Are Azure Services that are known and trusted for this resource type are allowed to bypass firewall configuration? See [Trusted Microsoft Services](https://github.com/MicrosoftDocs/azure-docs/blob/master/articles/service-bus-messaging/includes/service-bus-trusted-services.md)
    #[builder(into)]
    #[serde(rename = "trustedServicesAllowed")]
    pub r#trusted_services_allowed: Option<bool>,
}
