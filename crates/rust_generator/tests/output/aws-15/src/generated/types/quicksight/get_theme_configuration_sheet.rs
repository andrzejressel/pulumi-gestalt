#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetThemeConfigurationSheet {
    /// The layout options for tiles. See tile_layout.
    #[builder(into)]
    #[serde(rename = "tileLayouts")]
    pub r#tile_layouts: Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayout>,
    /// The display options for tiles. See tile.
    #[builder(into)]
    #[serde(rename = "tiles")]
    pub r#tiles: Vec<super::super::types::quicksight::GetThemeConfigurationSheetTile>,
}
