#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingCustomDomainCertVerificationDns {
    /// (Output)
    /// The last time Hosting checked your CustomDomain's DNS records.
    #[builder(into)]
    #[serde(rename = "checkTime")]
    pub r#check_time: Option<String>,
    /// A text string to serve at the path.
    #[builder(into)]
    #[serde(rename = "desireds")]
    pub r#desireds: Option<Vec<super::super::types::firebase::HostingCustomDomainCertVerificationDnsDesired>>,
    /// Whether Hosting was able to find the required file contents on the
    /// specified path during its last check.
    #[builder(into)]
    #[serde(rename = "discovereds")]
    pub r#discovereds: Option<Vec<super::super::types::firebase::HostingCustomDomainCertVerificationDnsDiscovered>>,
}
