#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsNielsenConfiguration {
    /// Enter the Distributor ID assigned to your organization by Nielsen.
    #[builder(into)]
    #[serde(rename = "distributorId")]
    pub r#distributor_id: Option<String>,
    /// Enables Nielsen PCM to ID3 tagging.
    #[builder(into)]
    #[serde(rename = "nielsenPcmToId3Tagging")]
    pub r#nielsen_pcm_to_id_3_tagging: Option<String>,
}
