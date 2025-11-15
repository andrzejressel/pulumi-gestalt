#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupDnsConfig {
    /// A list of nameservers the containers will search out to resolve requests. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nameservers")]
    pub r#nameservers: Vec<String>,
    /// A list of [resolver configuration options](https://man7.org/linux/man-pages/man5/resolv.conf.5.html). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Option<Vec<String>>,
    /// A list of search domains that DNS requests will search along. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "searchDomains")]
    pub r#search_domains: Option<Vec<String>>,
}
