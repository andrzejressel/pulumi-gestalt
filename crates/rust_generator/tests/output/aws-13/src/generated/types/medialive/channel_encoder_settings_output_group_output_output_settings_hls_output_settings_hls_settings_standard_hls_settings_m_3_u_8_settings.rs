#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3U8Settings {
    #[builder(into)]
    #[serde(rename = "audioFramesPerPes")]
    pub r#audio_frames_per_pes: Option<i32>,
    #[builder(into)]
    #[serde(rename = "audioPids")]
    pub r#audio_pids: Option<String>,
    #[builder(into)]
    #[serde(rename = "ecmPid")]
    pub r#ecm_pid: Option<String>,
    #[builder(into)]
    #[serde(rename = "nielsenId3Behavior")]
    pub r#nielsen_id_3_behavior: Option<String>,
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
    #[serde(rename = "scte35Behavior")]
    pub r#scte_35_behavior: Option<String>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Option<String>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3U8Settings {
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
                    "audio_frames_per_pes",
                    &self.r#audio_frames_per_pes,
                ),
                to_pulumi_object_field(
                    "audio_pids",
                    &self.r#audio_pids,
                ),
                to_pulumi_object_field(
                    "ecm_pid",
                    &self.r#ecm_pid,
                ),
                to_pulumi_object_field(
                    "nielsen_id_3_behavior",
                    &self.r#nielsen_id_3_behavior,
                ),
                to_pulumi_object_field(
                    "pat_interval",
                    &self.r#pat_interval,
                ),
                to_pulumi_object_field(
                    "pcr_control",
                    &self.r#pcr_control,
                ),
                to_pulumi_object_field(
                    "pcr_period",
                    &self.r#pcr_period,
                ),
                to_pulumi_object_field(
                    "pcr_pid",
                    &self.r#pcr_pid,
                ),
                to_pulumi_object_field(
                    "pmt_interval",
                    &self.r#pmt_interval,
                ),
                to_pulumi_object_field(
                    "pmt_pid",
                    &self.r#pmt_pid,
                ),
                to_pulumi_object_field(
                    "program_num",
                    &self.r#program_num,
                ),
                to_pulumi_object_field(
                    "scte_35_behavior",
                    &self.r#scte_35_behavior,
                ),
                to_pulumi_object_field(
                    "scte_35_pid",
                    &self.r#scte_35_pid,
                ),
                to_pulumi_object_field(
                    "timed_metadata_behavior",
                    &self.r#timed_metadata_behavior,
                ),
                to_pulumi_object_field(
                    "timed_metadata_pid",
                    &self.r#timed_metadata_pid,
                ),
                to_pulumi_object_field(
                    "transport_stream_id",
                    &self.r#transport_stream_id,
                ),
                to_pulumi_object_field(
                    "video_pid",
                    &self.r#video_pid,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettingsStandardHlsSettingsM3U8Settings {
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
                    r#ecm_pid: {
                        let field_value = match fields_map.get("ecm_pid") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecm_pid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scte_35_behavior: {
                        let field_value = match fields_map.get("scte_35_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_35_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
