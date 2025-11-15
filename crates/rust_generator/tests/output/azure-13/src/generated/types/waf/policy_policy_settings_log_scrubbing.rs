#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPolicySettingsLogScrubbing {
    /// Whether the log scrubbing is enabled or disabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// One or more `scrubbing_rule` blocks as define below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<super::super::types::waf::PolicyPolicySettingsLogScrubbingRule>>,
}
