#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetThumbnails {
    /// The aspect ratio of thumbnails. The following values are valid: auto, 1:1, 4:3, 3:2, 16:9
    #[builder(into)]
    #[serde(rename = "aspectRatio")]
    pub r#aspect_ratio: Option<String>,
    /// The format of thumbnails, if any. Valid formats are jpg and png.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// The approximate number of seconds between thumbnails. The value must be an integer. The actual interval can vary by several seconds from one thumbnail to the next.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<String>,
    /// The maximum height of thumbnails, in pixels. If you specify auto, Elastic Transcoder uses 1080 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 32 and 3072, inclusive.
    #[builder(into)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Option<String>,
    /// The maximum width of thumbnails, in pixels. If you specify auto, Elastic Transcoder uses 1920 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 32 and 4096, inclusive.
    #[builder(into)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Option<String>,
    /// When you set PaddingPolicy to Pad, Elastic Transcoder might add black bars to the top and bottom and/or left and right sides of thumbnails to make the total size of the thumbnails match the values that you specified for thumbnail MaxWidth and MaxHeight settings.
    #[builder(into)]
    #[serde(rename = "paddingPolicy")]
    pub r#padding_policy: Option<String>,
    /// The width and height of thumbnail files in pixels, in the format WidthxHeight, where both values are even integers. The values cannot exceed the width and height that you specified in the Video:Resolution object. (To better control resolution and aspect ratio of thumbnails, we recommend that you use the thumbnail values `max_width`, `max_height`, `sizing_policy`, and `padding_policy` instead of `resolution` and `aspect_ratio`. The two groups of settings are mutually exclusive. Do not use them together)
    #[builder(into)]
    #[serde(rename = "resolution")]
    pub r#resolution: Option<String>,
    /// A value that controls scaling of thumbnails. Valid values are: `Fit`, `Fill`, `Stretch`, `Keep`, `ShrinkToFit`, and `ShrinkToFill`.
    #[builder(into)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Option<String>,
}
