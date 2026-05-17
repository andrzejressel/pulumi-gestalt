#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobConfigElementaryStreamAudioStream {
    /// Audio bitrate in bits per second.
    #[builder(into)]
    #[serde(rename = "bitrateBps")]
    pub r#bitrate_bps: i32,
    /// Number of audio channels. The default is `2`.
    #[builder(into)]
    #[serde(rename = "channelCount")]
    pub r#channel_count: Option<i32>,
    /// A list of channel names specifying layout of the audio channels. The default is ["fl", "fr"].
    #[builder(into)]
    #[serde(rename = "channelLayouts")]
    pub r#channel_layouts: Option<Vec<String>>,
    /// The codec for this audio stream. The default is `aac`.
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Option<String>,
    /// The audio sample rate in Hertz. The default is `48000`.
    #[builder(into)]
    #[serde(rename = "sampleRateHertz")]
    pub r#sample_rate_hertz: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobConfigElementaryStreamAudioStream {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bitrate_bps",
                    &self.r#bitrate_bps,
                ),
                to_pulumi_object_field(
                    "channel_count",
                    &self.r#channel_count,
                ),
                to_pulumi_object_field(
                    "channel_layouts",
                    &self.r#channel_layouts,
                ),
                to_pulumi_object_field(
                    "codec",
                    &self.r#codec,
                ),
                to_pulumi_object_field(
                    "sample_rate_hertz",
                    &self.r#sample_rate_hertz,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobConfigElementaryStreamAudioStream {
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
                    r#bitrate_bps: {
                        let field_value = match fields_map.get("bitrate_bps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrate_bps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_count: {
                        let field_value = match fields_map.get("channel_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_layouts: {
                        let field_value = match fields_map.get("channel_layouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_layouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codec: {
                        let field_value = match fields_map.get("codec") {
                            Some(value) => value,
                            None => bail!("Missing field 'codec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_rate_hertz: {
                        let field_value = match fields_map.get("sample_rate_hertz") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_rate_hertz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
