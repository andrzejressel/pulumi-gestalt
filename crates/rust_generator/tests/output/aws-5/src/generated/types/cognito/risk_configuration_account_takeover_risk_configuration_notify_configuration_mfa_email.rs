#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail {
    /// The email HTML body.
    #[builder(into)]
    #[serde(rename = "htmlBody")]
    pub r#html_body: String,
    /// The email subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: String,
    /// The email text body.
    #[builder(into)]
    #[serde(rename = "textBody")]
    pub r#text_body: String,
}
