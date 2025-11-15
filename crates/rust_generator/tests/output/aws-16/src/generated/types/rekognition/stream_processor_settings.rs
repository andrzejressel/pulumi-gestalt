#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamProcessorSettings {
    /// Label detection settings to use on a streaming video. See `connected_home`.
    #[builder(into)]
    #[serde(rename = "connectedHome")]
    pub r#connected_home: Option<Box<super::super::types::rekognition::StreamProcessorSettingsConnectedHome>>,
    /// Input face recognition parameters for an Amazon Rekognition stream processor. See `face_search`.
    #[builder(into)]
    #[serde(rename = "faceSearch")]
    pub r#face_search: Option<Box<super::super::types::rekognition::StreamProcessorSettingsFaceSearch>>,
}
