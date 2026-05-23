#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateConfigElementaryStreamAudioStream {
    /// Audio bitrate in bits per second.
    #[builder(into)]
    pub r#bitrate_bps: i32,
    /// Number of audio channels. The default is `2`.
    #[builder(into)]
    pub r#channel_count: Option<i32>,
    /// A list of channel names specifying layout of the audio channels.  The default is ["fl", "fr"].
    #[builder(into)]
    pub r#channel_layouts: Option<Vec<String>>,
    /// The codec for this audio stream. The default is `aac`.
    #[builder(into)]
    pub r#codec: Option<String>,
    /// The audio sample rate in Hertz. The default is `48000`.
    #[builder(into)]
    pub r#sample_rate_hertz: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateConfigElementaryStreamAudioStream {
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
                    "bitrateBps",
                    &self.r#bitrate_bps,
                ),
                to_pulumi_object_field(
                    "channelCount",
                    &self.r#channel_count,
                ),
                to_pulumi_object_field(
                    "channelLayouts",
                    &self.r#channel_layouts,
                ),
                to_pulumi_object_field(
                    "codec",
                    &self.r#codec,
                ),
                to_pulumi_object_field(
                    "sampleRateHertz",
                    &self.r#sample_rate_hertz,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateConfigElementaryStreamAudioStream {
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
                        let field_value = match fields_map.get("bitrateBps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitrateBps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_count: {
                        let field_value = match fields_map.get("channelCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'channelCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channel_layouts: {
                        let field_value = match fields_map.get("channelLayouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'channelLayouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("sampleRateHertz") {
                            Some(value) => value,
                            None => bail!("Missing field 'sampleRateHertz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
