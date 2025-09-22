#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelInputAttachmentInputSettingsAudioSelectorSelectorSettingsAudioHlsRenditionSelection {
    /// Specifies the GROUP-ID in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    #[builder(into)]
    #[serde(rename = "groupId")]
    pub r#group_id: String,
    /// Specifies the NAME in the #EXT-X-MEDIA tag of the target HLS audio rendition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
