#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessApplicationLandingPageDesign {
    /// The button color of the landing page.
    #[builder(into)]
    #[serde(rename = "buttonColor")]
    pub r#button_color: Option<String>,
    /// The button text color of the landing page.
    #[builder(into)]
    #[serde(rename = "buttonTextColor")]
    pub r#button_text_color: Option<String>,
    /// The URL of the image to be displayed in the landing page.
    #[builder(into)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Option<String>,
    /// The message of the landing page.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The title of the landing page.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}
