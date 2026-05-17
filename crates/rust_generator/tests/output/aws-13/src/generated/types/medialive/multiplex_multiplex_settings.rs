#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiplexMultiplexSettings {
    /// Maximum video buffer delay.
    #[builder(into)]
    #[serde(rename = "maximumVideoBufferDelayMilliseconds")]
    pub r#maximum_video_buffer_delay_milliseconds: Option<i32>,
    /// Transport stream bit rate.
    #[builder(into)]
    #[serde(rename = "transportStreamBitrate")]
    pub r#transport_stream_bitrate: i32,
    /// Unique ID for each multiplex.
    #[builder(into)]
    #[serde(rename = "transportStreamId")]
    pub r#transport_stream_id: i32,
    /// Transport stream reserved bit rate.
    #[builder(into)]
    #[serde(rename = "transportStreamReservedBitrate")]
    pub r#transport_stream_reserved_bitrate: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MultiplexMultiplexSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "maximum_video_buffer_delay_milliseconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_video_buffer_delay_milliseconds,
                )
                .await,
            );
            map.insert(
                "transport_stream_bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transport_stream_bitrate,
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
                "transport_stream_reserved_bitrate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transport_stream_reserved_bitrate,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MultiplexMultiplexSettings {
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
                    r#maximum_video_buffer_delay_milliseconds: {
                        let field_value = match fields_map.get("maximum_video_buffer_delay_milliseconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_video_buffer_delay_milliseconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transport_stream_bitrate: {
                        let field_value = match fields_map.get("transport_stream_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport_stream_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#transport_stream_reserved_bitrate: {
                        let field_value = match fields_map.get("transport_stream_reserved_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'transport_stream_reserved_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
