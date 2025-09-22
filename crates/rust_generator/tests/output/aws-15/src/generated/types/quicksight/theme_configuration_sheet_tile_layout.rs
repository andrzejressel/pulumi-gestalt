#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThemeConfigurationSheetTileLayout {
    /// The gutter settings that apply between tiles. See gutter.
    #[builder(into)]
    #[serde(rename = "gutter")]
    pub r#gutter: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutGutter>>,
    /// The margin settings that apply around the outside edge of sheets. See margin.
    #[builder(into)]
    #[serde(rename = "margin")]
    pub r#margin: Option<Box<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutMargin>>,
}
