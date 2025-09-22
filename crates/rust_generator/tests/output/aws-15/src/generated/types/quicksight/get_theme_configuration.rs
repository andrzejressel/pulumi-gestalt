#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetThemeConfiguration {
    /// Color properties that apply to chart data colors. See data_color_palette.
    #[builder(into)]
    #[serde(rename = "dataColorPalettes")]
    pub r#data_color_palettes: Vec<super::super::types::quicksight::GetThemeConfigurationDataColorPalette>,
    /// Display options related to sheets. See sheet.
    #[builder(into)]
    #[serde(rename = "sheets")]
    pub r#sheets: Vec<super::super::types::quicksight::GetThemeConfigurationSheet>,
    /// Determines the typography options. See typography.
    #[builder(into)]
    #[serde(rename = "typographies")]
    pub r#typographies: Vec<super::super::types::quicksight::GetThemeConfigurationTypography>,
    /// Color properties that apply to the UI and to charts, excluding the colors that apply to data. See ui_color_palette.
    #[builder(into)]
    #[serde(rename = "uiColorPalettes")]
    pub r#ui_color_palettes: Vec<super::super::types::quicksight::GetThemeConfigurationUiColorPalette>,
}
