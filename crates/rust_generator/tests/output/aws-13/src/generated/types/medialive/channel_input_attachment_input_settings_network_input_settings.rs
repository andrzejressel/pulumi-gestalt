#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsNetworkInputSettings {
    /// Specifies HLS input settings when the uri is for a HLS manifest. See HLS Input Settings for more details.
    #[builder(into)]
    #[serde(rename = "hlsInputSettings")]
    pub r#hls_input_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsNetworkInputSettingsHlsInputSettings>>,
    /// Check HTTPS server certificates.
    #[builder(into)]
    #[serde(rename = "serverValidation")]
    pub r#server_validation: Option<String>,
}
