#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachment {
    /// User-specified settings for defining what the conditions are for declaring the input unhealthy and failing over to a different input. See Automatic Input Failover Settings for more details.
    #[builder(into)]
    #[serde(rename = "automaticInputFailoverSettings")]
    pub r#automatic_input_failover_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettings>>,
    /// User-specified name for the attachment.
    #[builder(into)]
    #[serde(rename = "inputAttachmentName")]
    pub r#input_attachment_name: String,
    /// The ID of the input.
    #[builder(into)]
    #[serde(rename = "inputId")]
    pub r#input_id: String,
    /// Settings of an input. See Input Settings for more details.
    #[builder(into)]
    #[serde(rename = "inputSettings")]
    pub r#input_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettings>>,
}
