#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OutboundConnectionRemoteDomainInfo {
    /// The name of the remote domain.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// The Account ID of the owner of the remote domain.
    #[builder(into)]
    #[serde(rename = "ownerId")]
    pub r#owner_id: String,
    /// The region of the remote domain.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
}
