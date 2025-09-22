#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamProcessorInput {
    /// Kinesis input stream. See `kinesis_video_stream`.
    #[builder(into)]
    #[serde(rename = "kinesisVideoStream")]
    pub r#kinesis_video_stream: Option<Box<super::super::types::rekognition::StreamProcessorInputKinesisVideoStream>>,
}
