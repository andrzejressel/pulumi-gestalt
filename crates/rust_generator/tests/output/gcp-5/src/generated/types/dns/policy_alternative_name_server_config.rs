#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyAlternativeNameServerConfig {
    /// Sets an alternative name server for the associated networks. When specified,
    /// all DNS queries are forwarded to a name server that you choose. Names such as .internal
    /// are not available when an alternative name server is specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "targetNameServers")]
    pub r#target_name_servers: Vec<super::super::types::dns::PolicyAlternativeNameServerConfigTargetNameServer>,
}
