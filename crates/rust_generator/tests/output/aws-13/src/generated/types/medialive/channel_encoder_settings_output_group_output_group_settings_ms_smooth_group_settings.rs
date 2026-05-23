#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
    #[builder(into)]
    pub r#acquisition_point_id: Option<String>,
    #[builder(into)]
    pub r#audio_only_timecode_control: Option<String>,
    /// Setting to allow self signed or verified RTMP certificates.
    #[builder(into)]
    pub r#certificate_mode: Option<String>,
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into)]
    pub r#connection_retry_interval: Option<i32>,
    #[builder(into)]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettingsDestination>,
    #[builder(into)]
    pub r#event_id: Option<String>,
    #[builder(into)]
    pub r#event_id_mode: Option<String>,
    #[builder(into)]
    pub r#event_stop_behavior: Option<String>,
    #[builder(into)]
    pub r#filecache_duration: Option<i32>,
    #[builder(into)]
    pub r#fragment_length: Option<i32>,
    #[builder(into)]
    pub r#input_loss_action: Option<String>,
    /// Number of retry attempts.
    #[builder(into)]
    pub r#num_retries: Option<i32>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    pub r#restart_delay: Option<i32>,
    #[builder(into)]
    pub r#segmentation_mode: Option<String>,
    #[builder(into)]
    pub r#send_delay_ms: Option<i32>,
    #[builder(into)]
    pub r#sparse_track_type: Option<String>,
    #[builder(into)]
    pub r#stream_manifest_behavior: Option<String>,
    #[builder(into)]
    pub r#timestamp_offset: Option<String>,
    #[builder(into)]
    pub r#timestamp_offset_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
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
                    "acquisitionPointId",
                    &self.r#acquisition_point_id,
                ),
                to_pulumi_object_field(
                    "audioOnlyTimecodeControl",
                    &self.r#audio_only_timecode_control,
                ),
                to_pulumi_object_field(
                    "certificateMode",
                    &self.r#certificate_mode,
                ),
                to_pulumi_object_field(
                    "connectionRetryInterval",
                    &self.r#connection_retry_interval,
                ),
                to_pulumi_object_field(
                    "destination",
                    &self.r#destination,
                ),
                to_pulumi_object_field(
                    "eventId",
                    &self.r#event_id,
                ),
                to_pulumi_object_field(
                    "eventIdMode",
                    &self.r#event_id_mode,
                ),
                to_pulumi_object_field(
                    "eventStopBehavior",
                    &self.r#event_stop_behavior,
                ),
                to_pulumi_object_field(
                    "filecacheDuration",
                    &self.r#filecache_duration,
                ),
                to_pulumi_object_field(
                    "fragmentLength",
                    &self.r#fragment_length,
                ),
                to_pulumi_object_field(
                    "inputLossAction",
                    &self.r#input_loss_action,
                ),
                to_pulumi_object_field(
                    "numRetries",
                    &self.r#num_retries,
                ),
                to_pulumi_object_field(
                    "restartDelay",
                    &self.r#restart_delay,
                ),
                to_pulumi_object_field(
                    "segmentationMode",
                    &self.r#segmentation_mode,
                ),
                to_pulumi_object_field(
                    "sendDelayMs",
                    &self.r#send_delay_ms,
                ),
                to_pulumi_object_field(
                    "sparseTrackType",
                    &self.r#sparse_track_type,
                ),
                to_pulumi_object_field(
                    "streamManifestBehavior",
                    &self.r#stream_manifest_behavior,
                ),
                to_pulumi_object_field(
                    "timestampOffset",
                    &self.r#timestamp_offset,
                ),
                to_pulumi_object_field(
                    "timestampOffsetMode",
                    &self.r#timestamp_offset_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings {
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
                    r#acquisition_point_id: {
                        let field_value = match fields_map.get("acquisitionPointId") {
                            Some(value) => value,
                            None => bail!("Missing field 'acquisitionPointId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_only_timecode_control: {
                        let field_value = match fields_map.get("audioOnlyTimecodeControl") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioOnlyTimecodeControl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_mode: {
                        let field_value = match fields_map.get("certificateMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificateMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_retry_interval: {
                        let field_value = match fields_map.get("connectionRetryInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionRetryInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#event_id: {
                        let field_value = match fields_map.get("eventId") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_id_mode: {
                        let field_value = match fields_map.get("eventIdMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventIdMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_stop_behavior: {
                        let field_value = match fields_map.get("eventStopBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventStopBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filecache_duration: {
                        let field_value = match fields_map.get("filecacheDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'filecacheDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fragment_length: {
                        let field_value = match fields_map.get("fragmentLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'fragmentLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#num_retries: {
                        let field_value = match fields_map.get("numRetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'numRetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restartDelay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restartDelay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segmentation_mode: {
                        let field_value = match fields_map.get("segmentationMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'segmentationMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#send_delay_ms: {
                        let field_value = match fields_map.get("sendDelayMs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sendDelayMs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sparse_track_type: {
                        let field_value = match fields_map.get("sparseTrackType") {
                            Some(value) => value,
                            None => bail!("Missing field 'sparseTrackType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_manifest_behavior: {
                        let field_value = match fields_map.get("streamManifestBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'streamManifestBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_offset: {
                        let field_value = match fields_map.get("timestampOffset") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampOffset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_offset_mode: {
                        let field_value = match fields_map.get("timestampOffsetMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampOffsetMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
