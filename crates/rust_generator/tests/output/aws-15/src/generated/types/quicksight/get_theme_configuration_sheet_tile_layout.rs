#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetThemeConfigurationSheetTileLayout {
    /// The gutter settings that apply between tiles. See gutter.
    #[builder(into)]
    #[serde(rename = "gutters")]
    pub r#gutters: Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayoutGutter>,
    /// The margin settings that apply around the outside edge of sheets. See margin.
    #[builder(into)]
    #[serde(rename = "margins")]
    pub r#margins: Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayoutMargin>,
}
