#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings {
    /// If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting “smart” justification will left-justify live subtitles and center-justify pre-recorded subtitles. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "alignment")]
    pub r#alignment: Option<String>,
    /// Specifies the color of the rectangle behind the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Option<String>,
    /// Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "backgroundOpacity")]
    pub r#background_opacity: Option<i32>,
    /// External font file used for caption burn-in. File extension must be ‘ttf’ or ‘tte’. Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts. All burn-in and DVB-Sub font settings must match. See Font for more details.
    #[builder(into)]
    #[serde(rename = "font")]
    pub r#font: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettingsFont>>,
    /// Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontColor")]
    pub r#font_color: Option<String>,
    /// Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontOpacity")]
    pub r#font_opacity: Option<i32>,
    /// Font resolution in DPI (dots per inch); default is 96 dpi. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontResolution")]
    pub r#font_resolution: Option<i32>,
    /// When set to ‘auto’ fontSize will scale depending on the size of the output. Giving a positive integer will specify the exact font size in points. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "fontSize")]
    pub r#font_size: Option<String>,
    /// Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "outlineColor")]
    pub r#outline_color: String,
    /// Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "outlineSize")]
    pub r#outline_size: Option<i32>,
    /// Specifies the color of the shadow cast by the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowColor")]
    pub r#shadow_color: Option<String>,
    /// Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowOpacity")]
    pub r#shadow_opacity: Option<i32>,
    /// Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowXOffset")]
    pub r#shadow_x_offset: Option<i32>,
    /// Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "shadowYOffset")]
    pub r#shadow_y_offset: Option<i32>,
    /// Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.
    #[builder(into)]
    #[serde(rename = "teletextGridControl")]
    pub r#teletext_grid_control: String,
    /// Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "xPosition")]
    pub r#x_position: Option<i32>,
    /// Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output. All burn-in and DVB-Sub font settings must match.
    #[builder(into)]
    #[serde(rename = "yPosition")]
    pub r#y_position: Option<i32>,
}
