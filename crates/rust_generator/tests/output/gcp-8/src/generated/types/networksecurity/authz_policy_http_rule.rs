#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyHttpRule {
    /// Describes properties of one or more sources of a request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: Option<Box<super::super::types::networksecurity::AuthzPolicyHttpRuleFrom>>,
    /// Describes properties of one or more targets of a request
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "to")]
    pub r#to: Option<Box<super::super::types::networksecurity::AuthzPolicyHttpRuleTo>>,
    /// CEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes.
    #[builder(into)]
    #[serde(rename = "when")]
    pub r#when: Option<String>,
}
