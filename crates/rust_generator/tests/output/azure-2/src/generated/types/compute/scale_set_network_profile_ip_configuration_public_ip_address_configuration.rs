#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetNetworkProfileIpConfigurationPublicIpAddressConfiguration {
    /// The domain name label for the DNS settings.
    #[builder(into)]
    #[serde(rename = "domainNameLabel")]
    pub r#domain_name_label: String,
    /// The idle timeout in minutes. This value must be between 4 and 30.
    #[builder(into)]
    #[serde(rename = "idleTimeout")]
    pub r#idle_timeout: i32,
    /// The name of the public IP address configuration
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
