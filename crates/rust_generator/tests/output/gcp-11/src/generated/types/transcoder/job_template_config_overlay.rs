#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfigOverlay {
    /// List of animations. The list should be chronological, without any time overlap.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "animations")]
    pub r#animations: Option<Vec<super::super::types::transcoder::JobTemplateConfigOverlayAnimation>>,
    /// Image overlay.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<Box<super::super::types::transcoder::JobTemplateConfigOverlayImage>>,
}
