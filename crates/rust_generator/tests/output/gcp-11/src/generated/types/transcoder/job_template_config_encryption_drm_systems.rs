#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigEncryptionDrmSystems {
    /// Clearkey configuration.
    #[builder(into)]
    #[serde(rename = "clearkey")]
    pub r#clearkey: Option<Box<super::super::types::transcoder::JobTemplateConfigEncryptionDrmSystemsClearkey>>,
    /// Fairplay configuration.
    #[builder(into)]
    #[serde(rename = "fairplay")]
    pub r#fairplay: Option<Box<super::super::types::transcoder::JobTemplateConfigEncryptionDrmSystemsFairplay>>,
    /// Playready configuration.
    #[builder(into)]
    #[serde(rename = "playready")]
    pub r#playready: Option<Box<super::super::types::transcoder::JobTemplateConfigEncryptionDrmSystemsPlayready>>,
    /// Widevine configuration.
    #[builder(into)]
    #[serde(rename = "widevine")]
    pub r#widevine: Option<Box<super::super::types::transcoder::JobTemplateConfigEncryptionDrmSystemsWidevine>>,
}
