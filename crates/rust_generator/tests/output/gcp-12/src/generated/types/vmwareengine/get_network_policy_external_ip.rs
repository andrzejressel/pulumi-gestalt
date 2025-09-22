#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkPolicyExternalIp {
    /// True if the service is enabled; false otherwise.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// State of the service. New values may be added to this enum when appropriate.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
}
