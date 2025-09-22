#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxPageEntryFulfillmentMessageText {
    /// (Output)
    /// Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request.
    #[builder(into)]
    #[serde(rename = "allowPlaybackInterruption")]
    pub r#allow_playback_interruption: Option<bool>,
    /// A collection of text responses.
    #[builder(into)]
    #[serde(rename = "texts")]
    pub r#texts: Option<Vec<String>>,
}
