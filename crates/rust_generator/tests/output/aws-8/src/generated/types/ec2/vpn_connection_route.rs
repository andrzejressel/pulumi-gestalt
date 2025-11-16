#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnConnectionRoute {
    /// The CIDR block associated with the local subnet of the customer data center.
    #[builder(into)]
    #[serde(rename = "destinationCidrBlock")]
    pub r#destination_cidr_block: Option<String>,
    /// Indicates how the routes were provided.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// The current state of the static route.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
