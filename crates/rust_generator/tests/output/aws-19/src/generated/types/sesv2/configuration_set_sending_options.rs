#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationSetSendingOptions {
    /// If `true`, email sending is enabled for the configuration set. If `false`, email sending is disabled for the configuration set.
    #[builder(into)]
    #[serde(rename = "sendingEnabled")]
    pub r#sending_enabled: Option<bool>,
}
