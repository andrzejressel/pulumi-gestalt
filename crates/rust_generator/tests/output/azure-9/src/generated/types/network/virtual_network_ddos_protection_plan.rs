#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkDdosProtectionPlan {
    /// Enable/disable DDoS Protection Plan on Virtual Network.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: bool,
    /// The ID of DDoS Protection Plan.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
