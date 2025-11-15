#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    /// The ad marker type for this output group.
    #[builder(into)]
    #[serde(rename = "adMarkers")]
    pub r#ad_markers: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "baseUrlContent")]
    pub r#base_url_content: Option<String>,
    #[builder(into)]
    #[serde(rename = "baseUrlContent1")]
    pub r#base_url_content_1: Option<String>,
    #[builder(into)]
    #[serde(rename = "baseUrlManifest")]
    pub r#base_url_manifest: Option<String>,
    #[builder(into)]
    #[serde(rename = "baseUrlManifest1")]
    pub r#base_url_manifest_1: Option<String>,
    #[builder(into)]
    #[serde(rename = "captionLanguageMappings")]
    pub r#caption_language_mappings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping>>,
    #[builder(into)]
    #[serde(rename = "captionLanguageSetting")]
    pub r#caption_language_setting: Option<String>,
    #[builder(into)]
    #[serde(rename = "clientCache")]
    pub r#client_cache: Option<String>,
    #[builder(into)]
    #[serde(rename = "codecSpecification")]
    pub r#codec_specification: Option<String>,
    #[builder(into)]
    #[serde(rename = "constantIv")]
    pub r#constant_iv: Option<String>,
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination>,
    #[builder(into)]
    #[serde(rename = "directoryStructure")]
    pub r#directory_structure: Option<String>,
    #[builder(into)]
    #[serde(rename = "discontinuityTags")]
    pub r#discontinuity_tags: Option<String>,
    #[builder(into)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "hlsCdnSettings")]
    pub r#hls_cdn_settings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting>>,
    #[builder(into)]
    #[serde(rename = "hlsId3SegmentTagging")]
    pub r#hls_id_3_segment_tagging: Option<String>,
    #[builder(into)]
    #[serde(rename = "iframeOnlyPlaylists")]
    pub r#iframe_only_playlists: Option<String>,
    #[builder(into)]
    #[serde(rename = "incompleteSegmentBehavior")]
    pub r#incomplete_segment_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "indexNSegments")]
    pub r#index_n_segments: Option<i32>,
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    #[builder(into)]
    #[serde(rename = "ivInManifest")]
    pub r#iv_in_manifest: Option<String>,
    #[builder(into)]
    #[serde(rename = "ivSource")]
    pub r#iv_source: Option<String>,
    #[builder(into)]
    #[serde(rename = "keepSegments")]
    pub r#keep_segments: Option<i32>,
    #[builder(into)]
    #[serde(rename = "keyFormat")]
    pub r#key_format: Option<String>,
    #[builder(into)]
    #[serde(rename = "keyFormatVersions")]
    pub r#key_format_versions: Option<String>,
    #[builder(into)]
    #[serde(rename = "keyProviderSettings")]
    pub r#key_provider_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings>>,
    #[builder(into)]
    #[serde(rename = "manifestCompression")]
    pub r#manifest_compression: Option<String>,
    #[builder(into)]
    #[serde(rename = "manifestDurationFormat")]
    pub r#manifest_duration_format: Option<String>,
    #[builder(into)]
    #[serde(rename = "minSegmentLength")]
    pub r#min_segment_length: Option<i32>,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "outputSelection")]
    pub r#output_selection: Option<String>,
    #[builder(into)]
    #[serde(rename = "programDateTime")]
    pub r#program_date_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "programDateTimeClock")]
    pub r#program_date_time_clock: Option<String>,
    #[builder(into)]
    #[serde(rename = "programDateTimePeriod")]
    pub r#program_date_time_period: Option<i32>,
    #[builder(into)]
    #[serde(rename = "redundantManifest")]
    pub r#redundant_manifest: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentLength")]
    pub r#segment_length: Option<i32>,
    #[builder(into)]
    #[serde(rename = "segmentsPerSubdirectory")]
    pub r#segments_per_subdirectory: Option<i32>,
    #[builder(into)]
    #[serde(rename = "streamInfResolution")]
    pub r#stream_inf_resolution: Option<String>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Frame")]
    pub r#timed_metadata_id_3_frame: Option<String>,
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Period")]
    pub r#timed_metadata_id_3_period: Option<i32>,
    #[builder(into)]
    #[serde(rename = "timestampDeltaMilliseconds")]
    pub r#timestamp_delta_milliseconds: Option<i32>,
    #[builder(into)]
    #[serde(rename = "tsFileMode")]
    pub r#ts_file_mode: Option<String>,
}
