#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsLocationNetwork {
    /// The ID of this resource.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// CIDR notation representation of the network IP.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
}
