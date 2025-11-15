#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPublicIPsPublicIp {
    /// The Domain Name Label of the Public IP Address
    #[builder(into)]
    #[serde(rename = "domainNameLabel")]
    pub r#domain_name_label: String,
    /// The FQDN of the Public IP Address
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: String,
    /// The ID of the Public IP Address
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The IP address of the Public IP Address
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// The Name of the Public IP Address
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
