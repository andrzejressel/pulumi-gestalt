#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers
    #[builder(into)]
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    /// A search list for host-name lookup
    #[builder(into, default)]
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}
