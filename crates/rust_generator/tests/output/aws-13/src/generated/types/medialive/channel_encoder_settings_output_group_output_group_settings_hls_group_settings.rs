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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ad_markers".to_string(), self.r#ad_markers.to_pulumi_value().await);
            map.insert("base_url_content".to_string(), self.r#base_url_content.to_pulumi_value().await);
            map.insert("base_url_content_1".to_string(), self.r#base_url_content_1.to_pulumi_value().await);
            map.insert("base_url_manifest".to_string(), self.r#base_url_manifest.to_pulumi_value().await);
            map.insert("base_url_manifest_1".to_string(), self.r#base_url_manifest_1.to_pulumi_value().await);
            map.insert("caption_language_mappings".to_string(), self.r#caption_language_mappings.to_pulumi_value().await);
            map.insert("caption_language_setting".to_string(), self.r#caption_language_setting.to_pulumi_value().await);
            map.insert("client_cache".to_string(), self.r#client_cache.to_pulumi_value().await);
            map.insert("codec_specification".to_string(), self.r#codec_specification.to_pulumi_value().await);
            map.insert("constant_iv".to_string(), self.r#constant_iv.to_pulumi_value().await);
            map.insert("destination".to_string(), self.r#destination.to_pulumi_value().await);
            map.insert("directory_structure".to_string(), self.r#directory_structure.to_pulumi_value().await);
            map.insert("discontinuity_tags".to_string(), self.r#discontinuity_tags.to_pulumi_value().await);
            map.insert("encryption_type".to_string(), self.r#encryption_type.to_pulumi_value().await);
            map.insert("hls_cdn_settings".to_string(), self.r#hls_cdn_settings.to_pulumi_value().await);
            map.insert("hls_id_3_segment_tagging".to_string(), self.r#hls_id_3_segment_tagging.to_pulumi_value().await);
            map.insert("iframe_only_playlists".to_string(), self.r#iframe_only_playlists.to_pulumi_value().await);
            map.insert("incomplete_segment_behavior".to_string(), self.r#incomplete_segment_behavior.to_pulumi_value().await);
            map.insert("index_n_segments".to_string(), self.r#index_n_segments.to_pulumi_value().await);
            map.insert("input_loss_action".to_string(), self.r#input_loss_action.to_pulumi_value().await);
            map.insert("iv_in_manifest".to_string(), self.r#iv_in_manifest.to_pulumi_value().await);
            map.insert("iv_source".to_string(), self.r#iv_source.to_pulumi_value().await);
            map.insert("keep_segments".to_string(), self.r#keep_segments.to_pulumi_value().await);
            map.insert("key_format".to_string(), self.r#key_format.to_pulumi_value().await);
            map.insert("key_format_versions".to_string(), self.r#key_format_versions.to_pulumi_value().await);
            map.insert("key_provider_settings".to_string(), self.r#key_provider_settings.to_pulumi_value().await);
            map.insert("manifest_compression".to_string(), self.r#manifest_compression.to_pulumi_value().await);
            map.insert("manifest_duration_format".to_string(), self.r#manifest_duration_format.to_pulumi_value().await);
            map.insert("min_segment_length".to_string(), self.r#min_segment_length.to_pulumi_value().await);
            map.insert("mode".to_string(), self.r#mode.to_pulumi_value().await);
            map.insert("output_selection".to_string(), self.r#output_selection.to_pulumi_value().await);
            map.insert("program_date_time".to_string(), self.r#program_date_time.to_pulumi_value().await);
            map.insert("program_date_time_clock".to_string(), self.r#program_date_time_clock.to_pulumi_value().await);
            map.insert("program_date_time_period".to_string(), self.r#program_date_time_period.to_pulumi_value().await);
            map.insert("redundant_manifest".to_string(), self.r#redundant_manifest.to_pulumi_value().await);
            map.insert("segment_length".to_string(), self.r#segment_length.to_pulumi_value().await);
            map.insert("segments_per_subdirectory".to_string(), self.r#segments_per_subdirectory.to_pulumi_value().await);
            map.insert("stream_inf_resolution".to_string(), self.r#stream_inf_resolution.to_pulumi_value().await);
            map.insert("timed_metadata_id_3_frame".to_string(), self.r#timed_metadata_id_3_frame.to_pulumi_value().await);
            map.insert("timed_metadata_id_3_period".to_string(), self.r#timed_metadata_id_3_period.to_pulumi_value().await);
            map.insert("timestamp_delta_milliseconds".to_string(), self.r#timestamp_delta_milliseconds.to_pulumi_value().await);
            map.insert("ts_file_mode".to_string(), self.r#ts_file_mode.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#ad_markers: {
                        let field_value = match fields_map.get("ad_markers") {
                            Some(value) => value,
                            None => bail!("Missing field 'ad_markers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#base_url_content: {
                        let field_value = match fields_map.get("base_url_content") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_url_content' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#base_url_content_1: {
                        let field_value = match fields_map.get("base_url_content_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_url_content_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#base_url_manifest: {
                        let field_value = match fields_map.get("base_url_manifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_url_manifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#base_url_manifest_1: {
                        let field_value = match fields_map.get("base_url_manifest_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_url_manifest_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caption_language_mappings: {
                        let field_value = match fields_map.get("caption_language_mappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_language_mappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caption_language_setting: {
                        let field_value = match fields_map.get("caption_language_setting") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_language_setting' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#client_cache: {
                        let field_value = match fields_map.get("client_cache") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_cache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#codec_specification: {
                        let field_value = match fields_map.get("codec_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'codec_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#constant_iv: {
                        let field_value = match fields_map.get("constant_iv") {
                            Some(value) => value,
                            None => bail!("Missing field 'constant_iv' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#directory_structure: {
                        let field_value = match fields_map.get("directory_structure") {
                            Some(value) => value,
                            None => bail!("Missing field 'directory_structure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#discontinuity_tags: {
                        let field_value = match fields_map.get("discontinuity_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'discontinuity_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption_type: {
                        let field_value = match fields_map.get("encryption_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_cdn_settings: {
                        let field_value = match fields_map.get("hls_cdn_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_cdn_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_id_3_segment_tagging: {
                        let field_value = match fields_map.get("hls_id_3_segment_tagging") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_id_3_segment_tagging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iframe_only_playlists: {
                        let field_value = match fields_map.get("iframe_only_playlists") {
                            Some(value) => value,
                            None => bail!("Missing field 'iframe_only_playlists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#incomplete_segment_behavior: {
                        let field_value = match fields_map.get("incomplete_segment_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'incomplete_segment_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#index_n_segments: {
                        let field_value = match fields_map.get("index_n_segments") {
                            Some(value) => value,
                            None => bail!("Missing field 'index_n_segments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_action: {
                        let field_value = match fields_map.get("input_loss_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iv_in_manifest: {
                        let field_value = match fields_map.get("iv_in_manifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'iv_in_manifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iv_source: {
                        let field_value = match fields_map.get("iv_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'iv_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#keep_segments: {
                        let field_value = match fields_map.get("keep_segments") {
                            Some(value) => value,
                            None => bail!("Missing field 'keep_segments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key_format: {
                        let field_value = match fields_map.get("key_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key_format_versions: {
                        let field_value = match fields_map.get("key_format_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_format_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key_provider_settings: {
                        let field_value = match fields_map.get("key_provider_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_provider_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#manifest_compression: {
                        let field_value = match fields_map.get("manifest_compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'manifest_compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#manifest_duration_format: {
                        let field_value = match fields_map.get("manifest_duration_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'manifest_duration_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_segment_length: {
                        let field_value = match fields_map.get("min_segment_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_segment_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_selection: {
                        let field_value = match fields_map.get("output_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#program_date_time: {
                        let field_value = match fields_map.get("program_date_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'program_date_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#program_date_time_clock: {
                        let field_value = match fields_map.get("program_date_time_clock") {
                            Some(value) => value,
                            None => bail!("Missing field 'program_date_time_clock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#program_date_time_period: {
                        let field_value = match fields_map.get("program_date_time_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'program_date_time_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#redundant_manifest: {
                        let field_value = match fields_map.get("redundant_manifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'redundant_manifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#segment_length: {
                        let field_value = match fields_map.get("segment_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#segments_per_subdirectory: {
                        let field_value = match fields_map.get("segments_per_subdirectory") {
                            Some(value) => value,
                            None => bail!("Missing field 'segments_per_subdirectory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stream_inf_resolution: {
                        let field_value = match fields_map.get("stream_inf_resolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_inf_resolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_frame: {
                        let field_value = match fields_map.get("timed_metadata_id_3_frame") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_id_3_frame' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_period: {
                        let field_value = match fields_map.get("timed_metadata_id_3_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_id_3_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timestamp_delta_milliseconds: {
                        let field_value = match fields_map.get("timestamp_delta_milliseconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_delta_milliseconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ts_file_mode: {
                        let field_value = match fields_map.get("ts_file_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'ts_file_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
