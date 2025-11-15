#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppIngressIpSecurityRestriction {
    /// The IP-filter action. `Allow` or `Deny`.
    /// 
    /// > **NOTE:** The `action` types in an all `ip_security_restriction` blocks must be the same for the `ingress`, mixing `Allow` and `Deny` rules is not currently supported by the service.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Describe the IP restriction rule that is being sent to the container-app.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The incoming IP address or range of IP addresses (in CIDR notation).
    #[builder(into)]
    #[serde(rename = "ipAddressRange")]
    pub r#ip_address_range: String,
    /// Name for the IP restriction rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
