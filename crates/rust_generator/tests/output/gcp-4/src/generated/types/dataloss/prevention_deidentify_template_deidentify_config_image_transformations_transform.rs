#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform {
    /// Apply transformation to all findings not specified in other ImageTransformation's selectedInfoTypes.
    #[builder(into)]
    #[serde(rename = "allInfoTypes")]
    pub r#all_info_types: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllInfoTypes>>,
    /// Apply transformation to all text that doesn't match an infoType.
    #[builder(into)]
    #[serde(rename = "allText")]
    pub r#all_text: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllText>>,
    /// The color to use when redacting content from an image. If not specified, the default is black.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redactionColor")]
    pub r#redaction_color: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor>>,
    /// Apply transformation to the selected infoTypes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedInfoTypes")]
    pub r#selected_info_types: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformSelectedInfoTypes>>,
}
