#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiTokenConditionRequestIp {
    /// List of IP addresses or CIDR notation where the token may be used from. If not specified, the token will be valid for all IP addresses.
    #[builder(into, default)]
    #[serde(rename = "ins")]
    pub r#ins: Box<Option<Vec<String>>>,
    /// List of IP addresses or CIDR notation where the token should not be used from.
    #[builder(into, default)]
    #[serde(rename = "notIns")]
    pub r#not_ins: Box<Option<Vec<String>>>,
}
