#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxFlowAdvancedSettingsLoggingSettings {
    /// Enables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted.
    #[builder(into)]
    #[serde(rename = "enableConsentBasedRedaction")]
    pub r#enable_consent_based_redaction: Option<bool>,
    /// Enables DF Interaction logging.
    #[builder(into)]
    #[serde(rename = "enableInteractionLogging")]
    pub r#enable_interaction_logging: Option<bool>,
    /// Enables Google Cloud Logging.
    #[builder(into)]
    #[serde(rename = "enableStackdriverLogging")]
    pub r#enable_stackdriver_logging: Option<bool>,
}
