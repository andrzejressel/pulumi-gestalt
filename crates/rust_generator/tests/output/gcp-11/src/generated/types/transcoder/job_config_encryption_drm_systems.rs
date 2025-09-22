#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobConfigEncryptionDrmSystems {
    /// Clearkey configuration.
    #[builder(into)]
    #[serde(rename = "clearkey")]
    pub r#clearkey: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsClearkey>>,
    /// Fairplay configuration.
    #[builder(into)]
    #[serde(rename = "fairplay")]
    pub r#fairplay: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsFairplay>>,
    /// Playready configuration.
    #[builder(into)]
    #[serde(rename = "playready")]
    pub r#playready: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsPlayready>>,
    /// Widevine configuration.
    #[builder(into)]
    #[serde(rename = "widevine")]
    pub r#widevine: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystemsWidevine>>,
}
