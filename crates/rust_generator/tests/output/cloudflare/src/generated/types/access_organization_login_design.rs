#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessOrganizationLoginDesign {
    /// The background color on the login page.
    #[builder(into)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    /// The text at the bottom of the login page.
    #[builder(into)]
    #[serde(rename = "footerText")]
    pub r#footer_text: Option<String>,
    /// The text at the top of the login page.
    #[builder(into)]
    #[serde(rename = "headerText")]
    pub r#header_text: Option<String>,
    /// The URL of the logo on the login page.
    #[builder(into)]
    #[serde(rename = "logoPath")]
    pub r#logo_path: Option<String>,
    /// The text color on the login page.
    #[builder(into)]
    #[serde(rename = "textColor")]
    pub r#text_color: Option<String>,
}
