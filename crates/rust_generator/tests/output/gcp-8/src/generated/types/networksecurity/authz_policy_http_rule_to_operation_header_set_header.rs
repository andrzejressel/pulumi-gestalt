#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthzPolicyHttpRuleToOperationHeaderSetHeader {
    /// Specifies the name of the header in the request.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Specifies how the header match will be performed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<super::super::types::networksecurity::AuthzPolicyHttpRuleToOperationHeaderSetHeaderValue>>,
}
