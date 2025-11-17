#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessLevelsAccessLevel {
    /// A set of predefined conditions for the access level and a combining function.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "basic")]
    pub r#basic: Option<Box<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasic>>,
    /// Custom access level conditions are set using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request.
    /// See CEL spec at: https://github.com/google/cel-spec.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "custom")]
    pub r#custom: Option<Box<super::super::types::accesscontextmanager::AccessLevelsAccessLevelCustom>>,
    /// Description of the AccessLevel and its use. Does not affect behavior.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Resource name for the Access Level. The short_name component must begin
    /// with a letter and only include alphanumeric and '_'.
    /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Human readable title. Must be unique within the Policy.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}
