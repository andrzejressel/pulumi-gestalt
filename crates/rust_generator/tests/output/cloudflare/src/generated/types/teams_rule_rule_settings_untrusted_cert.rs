#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsRuleRuleSettingsUntrustedCert {
    /// Action to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
}
