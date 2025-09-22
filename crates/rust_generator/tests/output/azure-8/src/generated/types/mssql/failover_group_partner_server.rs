#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FailoverGroupPartnerServer {
    /// The ID of a partner SQL server to include in the failover group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The location of the partner server.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// The replication role of the partner server. Possible values include `Primary` or `Secondary`.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Option<String>,
}
