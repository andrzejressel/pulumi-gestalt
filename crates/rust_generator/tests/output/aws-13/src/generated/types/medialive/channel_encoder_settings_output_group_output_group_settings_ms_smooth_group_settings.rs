#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
    #[builder(into)]
    #[serde(rename = "acquisitionPointId")]
    pub r#acquisition_point_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "audioOnlyTimecodeControl")]
    pub r#audio_only_timecode_control: Option<String>,
    /// Setting to allow self signed or verified RTMP certificates.
    #[builder(into)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Option<String>,
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination>,
    #[builder(into)]
    #[serde(rename = "eventId")]
    pub r#event_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "eventIdMode")]
    pub r#event_id_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "eventStopBehavior")]
    pub r#event_stop_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "fragmentLength")]
    pub r#fragment_length: Option<i32>,
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    /// Number of retry attempts.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Option<i32>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Option<i32>,
    #[builder(into)]
    #[serde(rename = "segmentationMode")]
    pub r#segmentation_mode: Option<String>,
    #[builder(into)]
    #[serde(rename = "sendDelayMs")]
    pub r#send_delay_ms: Option<i32>,
    #[builder(into)]
    #[serde(rename = "sparseTrackType")]
    pub r#sparse_track_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "streamManifestBehavior")]
    pub r#stream_manifest_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "timestampOffset")]
    pub r#timestamp_offset: Option<String>,
    #[builder(into)]
    #[serde(rename = "timestampOffsetMode")]
    pub r#timestamp_offset_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("acquisition_point_id".to_string(), self.r#acquisition_point_id.to_pulumi_value().await);
            map.insert("audio_only_timecode_control".to_string(), self.r#audio_only_timecode_control.to_pulumi_value().await);
            map.insert("certificate_mode".to_string(), self.r#certificate_mode.to_pulumi_value().await);
            map.insert("connection_retry_interval".to_string(), self.r#connection_retry_interval.to_pulumi_value().await);
            map.insert("destination".to_string(), self.r#destination.to_pulumi_value().await);
            map.insert("event_id".to_string(), self.r#event_id.to_pulumi_value().await);
            map.insert("event_id_mode".to_string(), self.r#event_id_mode.to_pulumi_value().await);
            map.insert("event_stop_behavior".to_string(), self.r#event_stop_behavior.to_pulumi_value().await);
            map.insert("filecache_duration".to_string(), self.r#filecache_duration.to_pulumi_value().await);
            map.insert("fragment_length".to_string(), self.r#fragment_length.to_pulumi_value().await);
            map.insert("input_loss_action".to_string(), self.r#input_loss_action.to_pulumi_value().await);
            map.insert("num_retries".to_string(), self.r#num_retries.to_pulumi_value().await);
            map.insert("restart_delay".to_string(), self.r#restart_delay.to_pulumi_value().await);
            map.insert("segmentation_mode".to_string(), self.r#segmentation_mode.to_pulumi_value().await);
            map.insert("send_delay_ms".to_string(), self.r#send_delay_ms.to_pulumi_value().await);
            map.insert("sparse_track_type".to_string(), self.r#sparse_track_type.to_pulumi_value().await);
            map.insert("stream_manifest_behavior".to_string(), self.r#stream_manifest_behavior.to_pulumi_value().await);
            map.insert("timestamp_offset".to_string(), self.r#timestamp_offset.to_pulumi_value().await);
            map.insert("timestamp_offset_mode".to_string(), self.r#timestamp_offset_mode.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
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
                    r#acquisition_point_id: {
                        let field_value = match fields_map.get("acquisition_point_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'acquisition_point_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#audio_only_timecode_control: {
                        let field_value = match fields_map.get("audio_only_timecode_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_only_timecode_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#certificate_mode: {
                        let field_value = match fields_map.get("certificate_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#connection_retry_interval: {
                        let field_value = match fields_map.get("connection_retry_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_retry_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#event_id: {
                        let field_value = match fields_map.get("event_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#event_id_mode: {
                        let field_value = match fields_map.get("event_id_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_id_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#event_stop_behavior: {
                        let field_value = match fields_map.get("event_stop_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_stop_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#filecache_duration: {
                        let field_value = match fields_map.get("filecache_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'filecache_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#fragment_length: {
                        let field_value = match fields_map.get("fragment_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'fragment_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#num_retries: {
                        let field_value = match fields_map.get("num_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restart_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restart_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#segmentation_mode: {
                        let field_value = match fields_map.get("segmentation_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentation_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#send_delay_ms: {
                        let field_value = match fields_map.get("send_delay_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'send_delay_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sparse_track_type: {
                        let field_value = match fields_map.get("sparse_track_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'sparse_track_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stream_manifest_behavior: {
                        let field_value = match fields_map.get("stream_manifest_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_manifest_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timestamp_offset: {
                        let field_value = match fields_map.get("timestamp_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timestamp_offset_mode: {
                        let field_value = match fields_map.get("timestamp_offset_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_offset_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
