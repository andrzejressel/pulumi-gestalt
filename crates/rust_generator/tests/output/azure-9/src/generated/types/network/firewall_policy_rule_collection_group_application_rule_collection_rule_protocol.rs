#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRuleProtocol {
    /// Port number of the protocol. Range is 0-64000.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// Protocol type. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
