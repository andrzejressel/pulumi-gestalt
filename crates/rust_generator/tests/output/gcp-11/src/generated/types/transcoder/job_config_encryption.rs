#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobConfigEncryption {
    /// Configuration for AES-128 encryption.
    #[builder(into)]
    #[serde(rename = "aes128")]
    pub r#aes_128: Option<Box<super::super::types::transcoder::JobConfigEncryptionAes128>>,
    /// DRM system(s) to use; at least one must be specified. If a DRM system is omitted, it is considered disabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "drmSystems")]
    pub r#drm_systems: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystems>>,
    /// Identifier for this set of encryption options.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Configuration for MPEG Common Encryption (MPEG-CENC).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mpegCenc")]
    pub r#mpeg_cenc: Option<Box<super::super::types::transcoder::JobConfigEncryptionMpegCenc>>,
    /// Configuration for SAMPLE-AES encryption.
    #[builder(into)]
    #[serde(rename = "sampleAes")]
    pub r#sample_aes: Option<Box<super::super::types::transcoder::JobConfigEncryptionSampleAes>>,
    /// Configuration for secrets stored in Google Secret Manager.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretManagerKeySource")]
    pub r#secret_manager_key_source: Option<Box<super::super::types::transcoder::JobConfigEncryptionSecretManagerKeySource>>,
}
