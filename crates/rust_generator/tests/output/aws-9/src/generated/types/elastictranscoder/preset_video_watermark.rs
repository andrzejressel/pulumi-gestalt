#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PresetVideoWatermark {
    /// The horizontal position of the watermark unless you specify a nonzero value for `horzontal_offset`.
    #[builder(into)]
    #[serde(rename = "horizontalAlign")]
    pub r#horizontal_align: Option<String>,
    /// The amount by which you want the horizontal position of the watermark to be offset from the position specified by `horizontal_align`.
    #[builder(into)]
    #[serde(rename = "horizontalOffset")]
    pub r#horizontal_offset: Option<String>,
    /// A unique identifier for the settings for one watermark. The value of Id can be up to 40 characters long. You can specify settings for up to four watermarks.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The maximum height of the watermark.
    #[builder(into)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Option<String>,
    /// The maximum width of the watermark.
    #[builder(into)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Option<String>,
    /// A percentage that indicates how much you want a watermark to obscure the video in the location where it appears.
    #[builder(into)]
    #[serde(rename = "opacity")]
    pub r#opacity: Option<String>,
    /// A value that controls scaling of the watermark. Valid values are: `Fit`, `Stretch`, `ShrinkToFit`
    #[builder(into)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Option<String>,
    /// A value that determines how Elastic Transcoder interprets values that you specified for `video_watermarks.horizontal_offset`, `video_watermarks.vertical_offset`, `video_watermarks.max_width`, and `video_watermarks.max_height`. Valid values are `Content` and `Frame`.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    /// The vertical position of the watermark unless you specify a nonzero value for `vertical_align`. Valid values are `Top`, `Bottom`, `Center`.
    #[builder(into)]
    #[serde(rename = "verticalAlign")]
    pub r#vertical_align: Option<String>,
    /// The amount by which you want the vertical position of the watermark to be offset from the position specified by `vertical_align`
    #[builder(into)]
    #[serde(rename = "verticalOffset")]
    pub r#vertical_offset: Option<String>,
}
