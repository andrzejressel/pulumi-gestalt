#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustAccessApplicationTargetCriteria {
    /// The port that the targets use for the chosen communication protocol. A port cannot be assigned to multiple protocols.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The communication protocol your application secures.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Contains a map of target attribute keys to target attribute values.
    #[builder(into)]
    #[serde(rename = "targetAttributes")]
    pub r#target_attributes: Vec<super::types::ZeroTrustAccessApplicationTargetCriteriaTargetAttribute>,
}
