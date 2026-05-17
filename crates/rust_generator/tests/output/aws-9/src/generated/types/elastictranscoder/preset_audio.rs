#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PresetAudio {
    /// The method of organizing audio channels and tracks. Use Audio:Channels to specify the number of channels in your output, and Audio:AudioPackingMode to specify the number of tracks and their relation to the channels. If you do not specify an Audio:AudioPackingMode, Elastic Transcoder uses SingleTrack.
    #[builder(into)]
    #[serde(rename = "audioPackingMode")]
    pub r#audio_packing_mode: Option<String>,
    /// The bit rate of the audio stream in the output file, in kilobits/second. Enter an integer between 64 and 320, inclusive.
    #[builder(into)]
    #[serde(rename = "bitRate")]
    pub r#bit_rate: Option<String>,
    /// The number of audio channels in the output file
    #[builder(into)]
    #[serde(rename = "channels")]
    pub r#channels: Option<String>,
    /// The audio codec for the output file. Valid values are `AAC`, `flac`, `mp2`, `mp3`, `pcm`, and `vorbis`.
    #[builder(into)]
    #[serde(rename = "codec")]
    pub r#codec: Option<String>,
    /// The sample rate of the audio stream in the output file, in hertz. Valid values are: `auto`, `22050`, `32000`, `44100`, `48000`, `96000`
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PresetAudio {
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
                "audio_packing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audio_packing_mode,
                )
                .await,
            );
            map.insert(
                "bit_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bit_rate,
                )
                .await,
            );
            map.insert(
                "channels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#channels,
                )
                .await,
            );
            map.insert(
                "codec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#codec,
                )
                .await,
            );
            map.insert(
                "sample_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_rate,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PresetAudio {
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
                    r#audio_packing_mode: {
                        let field_value = match fields_map.get("audio_packing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_packing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bit_rate: {
                        let field_value = match fields_map.get("bit_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'bit_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#channels: {
                        let field_value = match fields_map.get("channels") {
                            Some(value) => value,
                            None => bail!("Missing field 'channels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sample_rate: {
                        let field_value = match fields_map.get("sample_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
