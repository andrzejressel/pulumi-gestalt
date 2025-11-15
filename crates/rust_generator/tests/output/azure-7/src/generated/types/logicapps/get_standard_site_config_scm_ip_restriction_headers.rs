#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetStandardSiteConfigScmIpRestrictionHeaders {
    /// A list of allowed Azure FrontDoor IDs in UUID notation.
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Option<Vec<String>>,
    /// A list to allow the Azure FrontDoor health probe header.
    #[builder(into)]
    #[serde(rename = "xFdHealthProbe")]
    pub r#x_fd_health_probe: Option<String>,
    /// A list of allowed 'X-Forwarded-For' IPs in CIDR notation.
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Option<Vec<String>>,
    /// A list of allowed 'X-Forwarded-Host' domains.
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Option<Vec<String>>,
}
