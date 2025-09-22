#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcIpamPoolCidrCidrAuthorizationContext {
    /// The plain-text authorization message for the prefix and account.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The signed authorization message for the prefix and account.
    #[builder(into)]
    #[serde(rename = "signature")]
    pub r#signature: Option<String>,
}
