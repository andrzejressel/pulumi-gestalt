#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    /// The ad marker type for this output group.
    #[builder(into)]
    pub r#ad_markers: Option<Vec<String>>,
    #[builder(into)]
    pub r#base_url_content: Option<String>,
    #[builder(into)]
    pub r#base_url_content_1: Option<String>,
    #[builder(into)]
    pub r#base_url_manifest: Option<String>,
    #[builder(into)]
    pub r#base_url_manifest_1: Option<String>,
    #[builder(into)]
    pub r#caption_language_mappings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping>>,
    #[builder(into)]
    pub r#caption_language_setting: Option<String>,
    #[builder(into)]
    pub r#client_cache: Option<String>,
    #[builder(into)]
    pub r#codec_specification: Option<String>,
    #[builder(into)]
    pub r#constant_iv: Option<String>,
    #[builder(into)]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination>,
    #[builder(into)]
    pub r#directory_structure: Option<String>,
    #[builder(into)]
    pub r#discontinuity_tags: Option<String>,
    #[builder(into)]
    pub r#encryption_type: Option<String>,
    #[builder(into)]
    pub r#hls_cdn_settings: Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting>>,
    #[builder(into)]
    pub r#hls_id_3_segment_tagging: Option<String>,
    #[builder(into)]
    pub r#iframe_only_playlists: Option<String>,
    #[builder(into)]
    pub r#incomplete_segment_behavior: Option<String>,
    #[builder(into)]
    pub r#index_n_segments: Option<i32>,
    #[builder(into)]
    pub r#input_loss_action: Option<String>,
    #[builder(into)]
    pub r#iv_in_manifest: Option<String>,
    #[builder(into)]
    pub r#iv_source: Option<String>,
    #[builder(into)]
    pub r#keep_segments: Option<i32>,
    #[builder(into)]
    pub r#key_format: Option<String>,
    #[builder(into)]
    pub r#key_format_versions: Option<String>,
    #[builder(into)]
    pub r#key_provider_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings>>,
    #[builder(into)]
    pub r#manifest_compression: Option<String>,
    #[builder(into)]
    pub r#manifest_duration_format: Option<String>,
    #[builder(into)]
    pub r#min_segment_length: Option<i32>,
    #[builder(into)]
    pub r#mode: Option<String>,
    #[builder(into)]
    pub r#output_selection: Option<String>,
    #[builder(into)]
    pub r#program_date_time: Option<String>,
    #[builder(into)]
    pub r#program_date_time_clock: Option<String>,
    #[builder(into)]
    pub r#program_date_time_period: Option<i32>,
    #[builder(into)]
    pub r#redundant_manifest: Option<String>,
    #[builder(into)]
    pub r#segment_length: Option<i32>,
    #[builder(into)]
    pub r#segments_per_subdirectory: Option<i32>,
    #[builder(into)]
    pub r#stream_inf_resolution: Option<String>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into)]
    pub r#timed_metadata_id_3_frame: Option<String>,
    #[builder(into)]
    pub r#timed_metadata_id_3_period: Option<i32>,
    #[builder(into)]
    pub r#timestamp_delta_milliseconds: Option<i32>,
    #[builder(into)]
    pub r#ts_file_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "adMarkers",
                    &self.r#ad_markers,
                ),
                to_pulumi_object_field(
                    "baseUrlContent",
                    &self.r#base_url_content,
                ),
                to_pulumi_object_field(
                    "baseUrlContent1",
                    &self.r#base_url_content_1,
                ),
                to_pulumi_object_field(
                    "baseUrlManifest",
                    &self.r#base_url_manifest,
                ),
                to_pulumi_object_field(
                    "baseUrlManifest1",
                    &self.r#base_url_manifest_1,
                ),
                to_pulumi_object_field(
                    "captionLanguageMappings",
                    &self.r#caption_language_mappings,
                ),
                to_pulumi_object_field(
                    "captionLanguageSetting",
                    &self.r#caption_language_setting,
                ),
                to_pulumi_object_field(
                    "clientCache",
                    &self.r#client_cache,
                ),
                to_pulumi_object_field(
                    "codecSpecification",
                    &self.r#codec_specification,
                ),
                to_pulumi_object_field(
                    "constantIv",
                    &self.r#constant_iv,
                ),
                to_pulumi_object_field(
                    "destination",
                    &self.r#destination,
                ),
                to_pulumi_object_field(
                    "directoryStructure",
                    &self.r#directory_structure,
                ),
                to_pulumi_object_field(
                    "discontinuityTags",
                    &self.r#discontinuity_tags,
                ),
                to_pulumi_object_field(
                    "encryptionType",
                    &self.r#encryption_type,
                ),
                to_pulumi_object_field(
                    "hlsCdnSettings",
                    &self.r#hls_cdn_settings,
                ),
                to_pulumi_object_field(
                    "hlsId3SegmentTagging",
                    &self.r#hls_id_3_segment_tagging,
                ),
                to_pulumi_object_field(
                    "iframeOnlyPlaylists",
                    &self.r#iframe_only_playlists,
                ),
                to_pulumi_object_field(
                    "incompleteSegmentBehavior",
                    &self.r#incomplete_segment_behavior,
                ),
                to_pulumi_object_field(
                    "indexNSegments",
                    &self.r#index_n_segments,
                ),
                to_pulumi_object_field(
                    "inputLossAction",
                    &self.r#input_loss_action,
                ),
                to_pulumi_object_field(
                    "ivInManifest",
                    &self.r#iv_in_manifest,
                ),
                to_pulumi_object_field(
                    "ivSource",
                    &self.r#iv_source,
                ),
                to_pulumi_object_field(
                    "keepSegments",
                    &self.r#keep_segments,
                ),
                to_pulumi_object_field(
                    "keyFormat",
                    &self.r#key_format,
                ),
                to_pulumi_object_field(
                    "keyFormatVersions",
                    &self.r#key_format_versions,
                ),
                to_pulumi_object_field(
                    "keyProviderSettings",
                    &self.r#key_provider_settings,
                ),
                to_pulumi_object_field(
                    "manifestCompression",
                    &self.r#manifest_compression,
                ),
                to_pulumi_object_field(
                    "manifestDurationFormat",
                    &self.r#manifest_duration_format,
                ),
                to_pulumi_object_field(
                    "minSegmentLength",
                    &self.r#min_segment_length,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "outputSelection",
                    &self.r#output_selection,
                ),
                to_pulumi_object_field(
                    "programDateTime",
                    &self.r#program_date_time,
                ),
                to_pulumi_object_field(
                    "programDateTimeClock",
                    &self.r#program_date_time_clock,
                ),
                to_pulumi_object_field(
                    "programDateTimePeriod",
                    &self.r#program_date_time_period,
                ),
                to_pulumi_object_field(
                    "redundantManifest",
                    &self.r#redundant_manifest,
                ),
                to_pulumi_object_field(
                    "segmentLength",
                    &self.r#segment_length,
                ),
                to_pulumi_object_field(
                    "segmentsPerSubdirectory",
                    &self.r#segments_per_subdirectory,
                ),
                to_pulumi_object_field(
                    "streamInfResolution",
                    &self.r#stream_inf_resolution,
                ),
                to_pulumi_object_field(
                    "timedMetadataId3Frame",
                    &self.r#timed_metadata_id_3_frame,
                ),
                to_pulumi_object_field(
                    "timedMetadataId3Period",
                    &self.r#timed_metadata_id_3_period,
                ),
                to_pulumi_object_field(
                    "timestampDeltaMilliseconds",
                    &self.r#timestamp_delta_milliseconds,
                ),
                to_pulumi_object_field(
                    "tsFileMode",
                    &self.r#ts_file_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#ad_markers: {
                        let field_value = match fields_map.get("adMarkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'adMarkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_url_content: {
                        let field_value = match fields_map.get("baseUrlContent") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseUrlContent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_url_content_1: {
                        let field_value = match fields_map.get("baseUrlContent1") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseUrlContent1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_url_manifest: {
                        let field_value = match fields_map.get("baseUrlManifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseUrlManifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_url_manifest_1: {
                        let field_value = match fields_map.get("baseUrlManifest1") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseUrlManifest1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_language_mappings: {
                        let field_value = match fields_map.get("captionLanguageMappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'captionLanguageMappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_language_setting: {
                        let field_value = match fields_map.get("captionLanguageSetting") {
                            Some(value) => value,
                            None => bail!("Missing field 'captionLanguageSetting' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_cache: {
                        let field_value = match fields_map.get("clientCache") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientCache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codec_specification: {
                        let field_value = match fields_map.get("codecSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'codecSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#constant_iv: {
                        let field_value = match fields_map.get("constantIv") {
                            Some(value) => value,
                            None => bail!("Missing field 'constantIv' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directory_structure: {
                        let field_value = match fields_map.get("directoryStructure") {
                            Some(value) => value,
                            None => bail!("Missing field 'directoryStructure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#discontinuity_tags: {
                        let field_value = match fields_map.get("discontinuityTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'discontinuityTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_type: {
                        let field_value = match fields_map.get("encryptionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hls_cdn_settings: {
                        let field_value = match fields_map.get("hlsCdnSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hlsCdnSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hls_id_3_segment_tagging: {
                        let field_value = match fields_map.get("hlsId3SegmentTagging") {
                            Some(value) => value,
                            None => bail!("Missing field 'hlsId3SegmentTagging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iframe_only_playlists: {
                        let field_value = match fields_map.get("iframeOnlyPlaylists") {
                            Some(value) => value,
                            None => bail!("Missing field 'iframeOnlyPlaylists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incomplete_segment_behavior: {
                        let field_value = match fields_map.get("incompleteSegmentBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'incompleteSegmentBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index_n_segments: {
                        let field_value = match fields_map.get("indexNSegments") {
                            Some(value) => value,
                            None => bail!("Missing field 'indexNSegments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_action: {
                        let field_value = match fields_map.get("inputLossAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputLossAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iv_in_manifest: {
                        let field_value = match fields_map.get("ivInManifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'ivInManifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iv_source: {
                        let field_value = match fields_map.get("ivSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'ivSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keep_segments: {
                        let field_value = match fields_map.get("keepSegments") {
                            Some(value) => value,
                            None => bail!("Missing field 'keepSegments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_format: {
                        let field_value = match fields_map.get("keyFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_format_versions: {
                        let field_value = match fields_map.get("keyFormatVersions") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyFormatVersions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_provider_settings: {
                        let field_value = match fields_map.get("keyProviderSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyProviderSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manifest_compression: {
                        let field_value = match fields_map.get("manifestCompression") {
                            Some(value) => value,
                            None => bail!("Missing field 'manifestCompression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manifest_duration_format: {
                        let field_value = match fields_map.get("manifestDurationFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'manifestDurationFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_segment_length: {
                        let field_value = match fields_map.get("minSegmentLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'minSegmentLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_selection: {
                        let field_value = match fields_map.get("outputSelection") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputSelection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#program_date_time: {
                        let field_value = match fields_map.get("programDateTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'programDateTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#program_date_time_clock: {
                        let field_value = match fields_map.get("programDateTimeClock") {
                            Some(value) => value,
                            None => bail!("Missing field 'programDateTimeClock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#program_date_time_period: {
                        let field_value = match fields_map.get("programDateTimePeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'programDateTimePeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redundant_manifest: {
                        let field_value = match fields_map.get("redundantManifest") {
                            Some(value) => value,
                            None => bail!("Missing field 'redundantManifest' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segment_length: {
                        let field_value = match fields_map.get("segmentLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segments_per_subdirectory: {
                        let field_value = match fields_map.get("segmentsPerSubdirectory") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentsPerSubdirectory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_inf_resolution: {
                        let field_value = match fields_map.get("streamInfResolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'streamInfResolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_frame: {
                        let field_value = match fields_map.get("timedMetadataId3Frame") {
                            Some(value) => value,
                            None => bail!("Missing field 'timedMetadataId3Frame' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_period: {
                        let field_value = match fields_map.get("timedMetadataId3Period") {
                            Some(value) => value,
                            None => bail!("Missing field 'timedMetadataId3Period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_delta_milliseconds: {
                        let field_value = match fields_map.get("timestampDeltaMilliseconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampDeltaMilliseconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ts_file_mode: {
                        let field_value = match fields_map.get("tsFileMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'tsFileMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
