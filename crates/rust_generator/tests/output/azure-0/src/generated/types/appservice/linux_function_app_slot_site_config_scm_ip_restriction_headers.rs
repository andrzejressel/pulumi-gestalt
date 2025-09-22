#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxFunctionAppSlotSiteConfigScmIpRestrictionHeaders {
    /// Specifies a list of Azure Front Door IDs.
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Option<Vec<String>>,
    /// Specifies if a Front Door Health Probe should be expected. The only possible value is `1`.
    #[builder(into)]
    #[serde(rename = "xFdHealthProbe")]
    pub r#x_fd_health_probe: Option<String>,
    /// Specifies a list of addresses for which matching should be applied. Omitting this value means allow any.
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Option<Vec<String>>,
    /// Specifies a list of Hosts for which matching should be applied.
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Option<Vec<String>>,
}
