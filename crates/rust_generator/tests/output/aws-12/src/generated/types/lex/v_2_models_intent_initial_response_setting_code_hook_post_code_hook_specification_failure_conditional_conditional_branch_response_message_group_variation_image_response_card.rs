#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureConditionalConditionalBranchResponseMessageGroupVariationImageResponseCard {
    /// Configuration blocks for buttons that should be displayed on the response card. The arrangement of the buttons is determined by the platform that displays the button. See `button`.
    #[builder(into)]
    #[serde(rename = "buttons")]
    pub r#buttons: Option<Vec<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecificationFailureConditionalConditionalBranchResponseMessageGroupVariationImageResponseCardButton>>,
    /// URL of an image to display on the response card. The image URL must be publicly available so that the platform displaying the response card has access to the image.
    #[builder(into)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Option<String>,
    /// Subtitle to display on the response card. The format of the subtitle is determined by the platform displaying the response card.
    #[builder(into)]
    #[serde(rename = "subtitle")]
    pub r#subtitle: Option<String>,
    /// Title to display on the response card. The format of the title is determined by the platform displaying the response card.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}
