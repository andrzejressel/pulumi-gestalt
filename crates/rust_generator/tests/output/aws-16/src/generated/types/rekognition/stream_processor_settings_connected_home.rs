#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamProcessorSettingsConnectedHome {
    /// Specifies what you want to detect in the video, such as people, packages, or pets. The current valid labels you can include in this list are: `PERSON`, `PET`, `PACKAGE`, and `ALL`.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<String>>,
    /// Minimum confidence required to label an object in the video.
    #[builder(into)]
    #[serde(rename = "minConfidence")]
    pub r#min_confidence: Option<f64>,
}
