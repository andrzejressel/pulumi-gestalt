#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxSecuritySettingsAudioExportSettings {
    /// Filename pattern for exported audio.
    #[builder(into)]
    #[serde(rename = "audioExportPattern")]
    pub r#audio_export_pattern: Option<String>,
    /// File format for exported audio file. Currently only in telephony recordings.
    /// * MULAW: G.711 mu-law PCM with 8kHz sample rate.
    /// * MP3: MP3 file format.
    /// * OGG: OGG Vorbis.
    /// Possible values are: `MULAW`, `MP3`, `OGG`.
    #[builder(into)]
    #[serde(rename = "audioFormat")]
    pub r#audio_format: Option<String>,
    /// Enable audio redaction if it is true.
    #[builder(into)]
    #[serde(rename = "enableAudioRedaction")]
    pub r#enable_audio_redaction: Option<bool>,
    /// Cloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy.
    #[builder(into)]
    #[serde(rename = "gcsBucket")]
    pub r#gcs_bucket: Option<String>,
}
