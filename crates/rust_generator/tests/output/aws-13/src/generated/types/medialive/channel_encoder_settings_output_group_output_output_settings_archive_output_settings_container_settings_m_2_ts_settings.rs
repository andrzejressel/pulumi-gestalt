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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "absentInputAudioBehavior",
                    &self.r#absent_input_audio_behavior,
                ),
                to_pulumi_object_field(
                    "arib",
                    &self.r#arib,
                ),
                to_pulumi_object_field(
                    "aribCaptionsPid",
                    &self.r#arib_captions_pid,
                ),
                to_pulumi_object_field(
                    "aribCaptionsPidControl",
                    &self.r#arib_captions_pid_control,
                ),
                to_pulumi_object_field(
                    "audioBufferModel",
                    &self.r#audio_buffer_model,
                ),
                to_pulumi_object_field(
                    "audioFramesPerPes",
                    &self.r#audio_frames_per_pes,
                ),
                to_pulumi_object_field(
                    "audioPids",
                    &self.r#audio_pids,
                ),
                to_pulumi_object_field(
                    "audioStreamType",
                    &self.r#audio_stream_type,
                ),
                to_pulumi_object_field(
                    "bitrate",
                    &self.r#bitrate,
                ),
                to_pulumi_object_field(
                    "bufferModel",
                    &self.r#buffer_model,
                ),
                to_pulumi_object_field(
                    "ccDescriptor",
                    &self.r#cc_descriptor,
                ),
                to_pulumi_object_field(
                    "dvbNitSettings",
                    &self.r#dvb_nit_settings,
                ),
                to_pulumi_object_field(
                    "dvbSdtSettings",
                    &self.r#dvb_sdt_settings,
                ),
                to_pulumi_object_field(
                    "dvbSubPids",
                    &self.r#dvb_sub_pids,
                ),
                to_pulumi_object_field(
                    "dvbTdtSettings",
                    &self.r#dvb_tdt_settings,
                ),
                to_pulumi_object_field(
                    "dvbTeletextPid",
                    &self.r#dvb_teletext_pid,
                ),
                to_pulumi_object_field(
                    "ebif",
                    &self.r#ebif,
                ),
                to_pulumi_object_field(
                    "ebpAudioInterval",
                    &self.r#ebp_audio_interval,
                ),
                to_pulumi_object_field(
                    "ebpLookaheadMs",
                    &self.r#ebp_lookahead_ms,
                ),
                to_pulumi_object_field(
                    "ebpPlacement",
                    &self.r#ebp_placement,
                ),
                to_pulumi_object_field(
                    "ecmPid",
                    &self.r#ecm_pid,
                ),
                to_pulumi_object_field(
                    "esRateInPes",
                    &self.r#es_rate_in_pes,
                ),
                to_pulumi_object_field(
                    "etvPlatformPid",
                    &self.r#etv_platform_pid,
                ),
                to_pulumi_object_field(
                    "etvSignalPid",
                    &self.r#etv_signal_pid,
                ),
                to_pulumi_object_field(
                    "fragmentTime",
                    &self.r#fragment_time,
                ),
                to_pulumi_object_field(
                    "klv",
                    &self.r#klv,
                ),
                to_pulumi_object_field(
                    "klvDataPids",
                    &self.r#klv_data_pids,
                ),
                to_pulumi_object_field(
                    "nielsenId3Behavior",
                    &self.r#nielsen_id_3_behavior,
                ),
                to_pulumi_object_field(
                    "nullPacketBitrate",
                    &self.r#null_packet_bitrate,
                ),
                to_pulumi_object_field(
                    "patInterval",
                    &self.r#pat_interval,
                ),
                to_pulumi_object_field(
                    "pcrControl",
                    &self.r#pcr_control,
                ),
                to_pulumi_object_field(
                    "pcrPeriod",
                    &self.r#pcr_period,
                ),
                to_pulumi_object_field(
                    "pcrPid",
                    &self.r#pcr_pid,
                ),
                to_pulumi_object_field(
                    "pmtInterval",
                    &self.r#pmt_interval,
                ),
                to_pulumi_object_field(
                    "pmtPid",
                    &self.r#pmt_pid,
                ),
                to_pulumi_object_field(
                    "programNum",
                    &self.r#program_num,
                ),
                to_pulumi_object_field(
                    "rateMode",
                    &self.r#rate_mode,
                ),
                to_pulumi_object_field(
                    "scte27Pids",
                    &self.r#scte_27_pids,
                ),
                to_pulumi_object_field(
                    "scte35Control",
                    &self.r#scte_35_control,
                ),
                to_pulumi_object_field(
                    "scte35Pid",
                    &self.r#scte_35_pid,
                ),
                to_pulumi_object_field(
                    "segmentationMarkers",
                    &self.r#segmentation_markers,
                ),
                to_pulumi_object_field(
                    "segmentationStyle",
                    &self.r#segmentation_style,
                ),
                to_pulumi_object_field(
                    "segmentationTime",
                    &self.r#segmentation_time,
                ),
                to_pulumi_object_field(
                    "timedMetadataBehavior",
                    &self.r#timed_metadata_behavior,
                ),
                to_pulumi_object_field(
                    "timedMetadataPid",
                    &self.r#timed_metadata_pid,
                ),
                to_pulumi_object_field(
                    "transportStreamId",
                    &self.r#transport_stream_id,
                ),
                to_pulumi_object_field(
                    "videoPid",
                    &self.r#video_pid,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("absentInputAudioBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'absentInputAudioBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("aribCaptionsPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'aribCaptionsPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arib_captions_pid_control: {
                        let field_value = match fields_map.get("aribCaptionsPidControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'aribCaptionsPidControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_buffer_model: {
                        let field_value = match fields_map.get("audioBufferModel") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioBufferModel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_frames_per_pes: {
                        let field_value = match fields_map.get("audioFramesPerPes") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioFramesPerPes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_pids: {
                        let field_value = match fields_map.get("audioPids") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioPids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_stream_type: {
                        let field_value = match fields_map.get("audioStreamType") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioStreamType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("bufferModel") {
                            Some(value) => value,
                            None => bail!("Missing field 'bufferModel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cc_descriptor: {
                        let field_value = match fields_map.get("ccDescriptor") {
                            Some(value) => value,
                            None => bail!("Missing field 'ccDescriptor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_nit_settings: {
                        let field_value = match fields_map.get("dvbNitSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbNitSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sdt_settings: {
                        let field_value = match fields_map.get("dvbSdtSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbSdtSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_pids: {
                        let field_value = match fields_map.get("dvbSubPids") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbSubPids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_tdt_settings: {
                        let field_value = match fields_map.get("dvbTdtSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbTdtSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_teletext_pid: {
                        let field_value = match fields_map.get("dvbTeletextPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbTeletextPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("ebpAudioInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebpAudioInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebp_lookahead_ms: {
                        let field_value = match fields_map.get("ebpLookaheadMs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebpLookaheadMs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebp_placement: {
                        let field_value = match fields_map.get("ebpPlacement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebpPlacement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecm_pid: {
                        let field_value = match fields_map.get("ecmPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecmPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#es_rate_in_pes: {
                        let field_value = match fields_map.get("esRateInPes") {
                            Some(value) => value,
                            None => bail!("Missing field 'esRateInPes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etv_platform_pid: {
                        let field_value = match fields_map.get("etvPlatformPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'etvPlatformPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#etv_signal_pid: {
                        let field_value = match fields_map.get("etvSignalPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'etvSignalPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fragment_time: {
                        let field_value = match fields_map.get("fragmentTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'fragmentTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("klvDataPids") {
                            Some(value) => value,
                            None => bail!("Missing field 'klvDataPids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nielsen_id_3_behavior: {
                        let field_value = match fields_map.get("nielsenId3Behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'nielsenId3Behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#null_packet_bitrate: {
                        let field_value = match fields_map.get("nullPacketBitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'nullPacketBitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pat_interval: {
                        let field_value = match fields_map.get("patInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'patInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_control: {
                        let field_value = match fields_map.get("pcrControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcrControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_period: {
                        let field_value = match fields_map.get("pcrPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcrPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcr_pid: {
                        let field_value = match fields_map.get("pcrPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcrPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pmt_interval: {
                        let field_value = match fields_map.get("pmtInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'pmtInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pmt_pid: {
                        let field_value = match fields_map.get("pmtPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'pmtPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#program_num: {
                        let field_value = match fields_map.get("programNum") {
                            Some(value) => value,
                            None => bail!("Missing field 'programNum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_mode: {
                        let field_value = match fields_map.get("rateMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_27_pids: {
                        let field_value = match fields_map.get("scte27Pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte27Pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_35_control: {
                        let field_value = match fields_map.get("scte35Control") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte35Control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_35_pid: {
                        let field_value = match fields_map.get("scte35Pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte35Pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_markers: {
                        let field_value = match fields_map.get("segmentationMarkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentationMarkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_style: {
                        let field_value = match fields_map.get("segmentationStyle") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentationStyle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_time: {
                        let field_value = match fields_map.get("segmentationTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentationTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_behavior: {
                        let field_value = match fields_map.get("timedMetadataBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'timedMetadataBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_pid: {
                        let field_value = match fields_map.get("timedMetadataPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'timedMetadataPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport_stream_id: {
                        let field_value = match fields_map.get("transportStreamId") {
                            Some(value) => value,
                            None => bail!("Missing field 'transportStreamId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_pid: {
                        let field_value = match fields_map.get("videoPid") {
                            Some(value) => value,
                            None => bail!("Missing field 'videoPid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
