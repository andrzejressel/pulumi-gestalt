#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewaySettingsBlockPage {
    /// Hex code of block page background color.
    #[builder(into)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    /// Indicator of enablement.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Block page footer text.
    #[builder(into)]
    #[serde(rename = "footerText")]
    pub r#footer_text: Option<String>,
    /// Block page header text.
    #[builder(into)]
    #[serde(rename = "headerText")]
    pub r#header_text: Option<String>,
    /// URL of block page logo.
    #[builder(into)]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Option<String>,
    /// Admin email for users to contact.
    #[builder(into)]
    #[serde(rename = "mailtoAddress")]
    pub r#mailto_address: Option<String>,
    /// Subject line for emails created from block page.
    #[builder(into)]
    #[serde(rename = "mailtoSubject")]
    pub r#mailto_subject: Option<String>,
    /// Name of block page configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
