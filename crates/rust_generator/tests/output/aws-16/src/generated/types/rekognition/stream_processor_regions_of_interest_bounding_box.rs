#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamProcessorRegionsOfInterestBoundingBox {
    /// Height of the bounding box as a ratio of the overall image height.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: Option<f64>,
    /// Left coordinate of the bounding box as a ratio of overall image width.
    #[builder(into)]
    #[serde(rename = "left")]
    pub r#left: Option<f64>,
    /// Top coordinate of the bounding box as a ratio of overall image height.
    #[builder(into)]
    #[serde(rename = "top")]
    pub r#top: Option<f64>,
    /// Width of the bounding box as a ratio of the overall image width.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Option<f64>,
}
