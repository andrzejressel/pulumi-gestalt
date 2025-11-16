#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKeyVaultNetworkAcl {
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: String,
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Vec<String>,
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Vec<String>,
}
