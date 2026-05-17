#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettings {
    #[builder(into)]
    #[serde(rename = "absentInputAudioBehavior")]
    pub r#absent_input_audio_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "arib")]
    pub r#arib: Option<String>,
    #[builder(into)]
    #[serde(rename = "aribCaptionsPid")]
    pub r#arib_captions_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "aribCaptionsPidControl")]
    pub r#arib_captions_pid_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioBufferModel")]
    pub r#audio_buffer_model: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioFramesPerPes")]
    pub r#audio_frames_per_pes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "audioPids")]
    pub r#audio_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioStreamType")]
    pub r#audio_stream_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Option<i32>,
    #[builder(into)]
    #[serde(rename = "bufferModel")]
    pub r#buffer_model: Option<String>,
    #[builder(into)]
    #[serde(rename = "ccDescriptor")]
    pub r#cc_descriptor: Option<String>,
    #[builder(into)]
    #[serde(rename = "dvbNitSettings")]
    pub r#dvb_nit_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettingsDvbNitSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbSdtSettings")]
    pub r#dvb_sdt_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbSubPids")]
    pub r#dvb_sub_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "dvbTdtSettings")]
    pub r#dvb_tdt_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettingsDvbTdtSettings>>,
    #[builder(into)]
    #[serde(rename = "dvbTeletextPid")]
    pub r#dvb_teletext_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebif")]
    pub r#ebif: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebpAudioInterval")]
    pub r#ebp_audio_interval: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebpLookaheadMs")]
    pub r#ebp_lookahead_ms: Option<i32>,
    #[builder(into)]
    #[serde(rename = "ebpPlacement")]
    pub r#ebp_placement: Option<String>,
    #[builder(into)]
    #[serde(rename = "ecmPid")]
    pub r#ecm_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "esRateInPes")]
    pub r#es_rate_in_pes: Option<String>,
    #[builder(into)]
    #[serde(rename = "etvPlatformPid")]
    pub r#etv_platform_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "etvSignalPid")]
    pub r#etv_signal_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "fragmentTime")]
    pub r#fragment_time: Option<f64>,
    #[builder(into)]
    #[serde(rename = "klv")]
    pub r#klv: Option<String>,
    #[builder(into)]
    #[serde(rename = "klvDataPids")]
    pub r#klv_data_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "nullPacketBitrate")]
    pub r#null_packet_bitrate: Option<f64>,
    #[builder(into)]
    #[serde(rename = "patInterval")]
    pub r#pat_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pcrControl")]
    pub r#pcr_control: Option<String>,
    #[builder(into)]
    #[serde(rename = "pcrPeriod")]
    pub r#pcr_period: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pcrPid")]
    pub r#pcr_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "pmtInterval")]
    pub r#pmt_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "pmtPid")]
    pub r#pmt_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "programNum")]
    pub r#program_num: Option<i32>,
    #[builder(into)]
    #[serde(rename = "rateMode")]
    pub r#rate_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "scte27Pids")]
    pub r#scte_27_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "scte35Control")]
    pub r#scte_35_control: Option<String>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationMarkers")]
    pub r#segmentation_markers: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationStyle")]
    pub r#segmentation_style: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentationTime")]
    pub r#segmentation_time: Option<f64>,
    #[builder(into)]
    #[serde(rename = "timedMetadataBehavior")]
    pub r#timed_metadata_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "timedMetadataPid")]
    pub r#timed_metadata_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: Option<i32>,
    #[builder(into)]
    #[serde(rename = "videoPid")]
    pub r#video_pid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "absent_input_audio_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#absent_input_audio_behavior,
                )
                .await,
            );
            map.insert(
                "arib".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arib,
                )
                .await,
            );
            map.insert(
                "arib_captions_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arib_captions_pid,
                )
                .await,
            );
            map.insert(
                "arib_captions_pid_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arib_captions_pid_control,
                )
                .await,
            );
            map.insert(
                "audio_buffer_model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_buffer_model,
                )
                .await,
            );
            map.insert(
                "audio_frames_per_pes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_frames_per_pes,
                )
                .await,
            );
            map.insert(
                "audio_pids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_pids,
                )
                .await,
            );
            map.insert(
                "audio_stream_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_stream_type,
                )
                .await,
            );
            map.insert(
                "bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bitrate,
                )
                .await,
            );
            map.insert(
                "buffer_model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffer_model,
                )
                .await,
            );
            map.insert(
                "cc_descriptor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cc_descriptor,
                )
                .await,
            );
            map.insert(
                "dvb_nit_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dvb_nit_settings,
                )
                .await,
            );
            map.insert(
                "dvb_sdt_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dvb_sdt_settings,
                )
                .await,
            );
            map.insert(
                "dvb_sub_pids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dvb_sub_pids,
                )
                .await,
            );
            map.insert(
                "dvb_tdt_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dvb_tdt_settings,
                )
                .await,
            );
            map.insert(
                "dvb_teletext_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dvb_teletext_pid,
                )
                .await,
            );
            map.insert(
                "ebif".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebif,
                )
                .await,
            );
            map.insert(
                "ebp_audio_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebp_audio_interval,
                )
                .await,
            );
            map.insert(
                "ebp_lookahead_ms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebp_lookahead_ms,
                )
                .await,
            );
            map.insert(
                "ebp_placement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ebp_placement,
                )
                .await,
            );
            map.insert(
                "ecm_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ecm_pid,
                )
                .await,
            );
            map.insert(
                "es_rate_in_pes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#es_rate_in_pes,
                )
                .await,
            );
            map.insert(
                "etv_platform_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#etv_platform_pid,
                )
                .await,
            );
            map.insert(
                "etv_signal_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#etv_signal_pid,
                )
                .await,
            );
            map.insert(
                "fragment_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fragment_time,
                )
                .await,
            );
            map.insert(
                "klv".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#klv,
                )
                .await,
            );
            map.insert(
                "klv_data_pids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#klv_data_pids,
                )
                .await,
            );
            map.insert(
                "nielsen_id_3_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nielsen_id_3_behavior,
                )
                .await,
            );
            map.insert(
                "null_packet_bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#null_packet_bitrate,
                )
                .await,
            );
            map.insert(
                "pat_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pat_interval,
                )
                .await,
            );
            map.insert(
                "pcr_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pcr_control,
                )
                .await,
            );
            map.insert(
                "pcr_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pcr_period,
                )
                .await,
            );
            map.insert(
                "pcr_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pcr_pid,
                )
                .await,
            );
            map.insert(
                "pmt_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pmt_interval,
                )
                .await,
            );
            map.insert(
                "pmt_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pmt_pid,
                )
                .await,
            );
            map.insert(
                "program_num".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#program_num,
                )
                .await,
            );
            map.insert(
                "rate_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_mode,
                )
                .await,
            );
            map.insert(
                "scte_27_pids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scte_27_pids,
                )
                .await,
            );
            map.insert(
                "scte_35_control".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scte_35_control,
                )
                .await,
            );
            map.insert(
                "scte_35_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scte_35_pid,
                )
                .await,
            );
            map.insert(
                "segmentation_markers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#segmentation_markers,
                )
                .await,
            );
            map.insert(
                "segmentation_style".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#segmentation_style,
                )
                .await,
            );
            map.insert(
                "segmentation_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#segmentation_time,
                )
                .await,
            );
            map.insert(
                "timed_metadata_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timed_metadata_behavior,
                )
                .await,
            );
            map.insert(
                "timed_metadata_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timed_metadata_pid,
                )
                .await,
            );
            map.insert(
                "transport_stream_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transport_stream_id,
                )
                .await,
            );
            map.insert(
                "video_pid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#video_pid,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettings {
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
                    r#absent_input_audio_behavior: {
                        let field_value = match fields_map.get("absent_input_audio_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'absent_input_audio_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arib: {
                        let field_value = match fields_map.get("arib") {
                            Some(value) => value,
                            None => bail!("Missing field 'arib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arib_captions_pid: {
                        let field_value = match fields_map.get("arib_captions_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'arib_captions_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arib_captions_pid_control: {
                        let field_value = match fields_map.get("arib_captions_pid_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'arib_captions_pid_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_buffer_model: {
                        let field_value = match fields_map.get("audio_buffer_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_buffer_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_frames_per_pes: {
                        let field_value = match fields_map.get("audio_frames_per_pes") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_frames_per_pes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_pids: {
                        let field_value = match fields_map.get("audio_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_stream_type: {
                        let field_value = match fields_map.get("audio_stream_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_stream_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bitrate: {
                        let field_value = match fields_map.get("bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffer_model: {
                        let field_value = match fields_map.get("buffer_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffer_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cc_descriptor: {
                        let field_value = match fields_map.get("cc_descriptor") {
                            Some(value) => value,
                            None => bail!("Missing field 'cc_descriptor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_nit_settings: {
                        let field_value = match fields_map.get("dvb_nit_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_nit_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sdt_settings: {
                        let field_value = match fields_map.get("dvb_sdt_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_sdt_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_pids: {
                        let field_value = match fields_map.get("dvb_sub_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_sub_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_tdt_settings: {
                        let field_value = match fields_map.get("dvb_tdt_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_tdt_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_teletext_pid: {
                        let field_value = match fields_map.get("dvb_teletext_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_teletext_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebif: {
                        let field_value = match fields_map.get("ebif") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebif' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebp_audio_interval: {
                        let field_value = match fields_map.get("ebp_audio_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebp_audio_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebp_lookahead_ms: {
                        let field_value = match fields_map.get("ebp_lookahead_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebp_lookahead_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebp_placement: {
                        let field_value = match fields_map.get("ebp_placement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebp_placement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecm_pid: {
                        let field_value = match fields_map.get("ecm_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecm_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#es_rate_in_pes: {
                        let field_value = match fields_map.get("es_rate_in_pes") {
                            Some(value) => value,
                            None => bail!("Missing field 'es_rate_in_pes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etv_platform_pid: {
                        let field_value = match fields_map.get("etv_platform_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'etv_platform_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etv_signal_pid: {
                        let field_value = match fields_map.get("etv_signal_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'etv_signal_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fragment_time: {
                        let field_value = match fields_map.get("fragment_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'fragment_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#klv: {
                        let field_value = match fields_map.get("klv") {
                            Some(value) => value,
                            None => bail!("Missing field 'klv' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#klv_data_pids: {
                        let field_value = match fields_map.get("klv_data_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'klv_data_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nielsen_id_3_behavior: {
                        let field_value = match fields_map.get("nielsen_id_3_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'nielsen_id_3_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#null_packet_bitrate: {
                        let field_value = match fields_map.get("null_packet_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'null_packet_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pat_interval: {
                        let field_value = match fields_map.get("pat_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'pat_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_control: {
                        let field_value = match fields_map.get("pcr_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcr_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_period: {
                        let field_value = match fields_map.get("pcr_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcr_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_pid: {
                        let field_value = match fields_map.get("pcr_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcr_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pmt_interval: {
                        let field_value = match fields_map.get("pmt_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'pmt_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pmt_pid: {
                        let field_value = match fields_map.get("pmt_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'pmt_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#program_num: {
                        let field_value = match fields_map.get("program_num") {
                            Some(value) => value,
                            None => bail!("Missing field 'program_num' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_mode: {
                        let field_value = match fields_map.get("rate_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_27_pids: {
                        let field_value = match fields_map.get("scte_27_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_27_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_35_control: {
                        let field_value = match fields_map.get("scte_35_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_35_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_35_pid: {
                        let field_value = match fields_map.get("scte_35_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_35_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_markers: {
                        let field_value = match fields_map.get("segmentation_markers") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentation_markers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_style: {
                        let field_value = match fields_map.get("segmentation_style") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentation_style' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_time: {
                        let field_value = match fields_map.get("segmentation_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentation_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_behavior: {
                        let field_value = match fields_map.get("timed_metadata_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_pid: {
                        let field_value = match fields_map.get("timed_metadata_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport_stream_id: {
                        let field_value = match fields_map.get("transport_stream_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport_stream_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_pid: {
                        let field_value = match fields_map.get("video_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
