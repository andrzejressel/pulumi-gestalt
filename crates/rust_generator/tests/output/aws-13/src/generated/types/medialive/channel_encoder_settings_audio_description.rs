#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescription {
    /// Advanced audio normalization settings. See Audio Normalization Settings for more details.
    #[builder(into)]
    #[serde(rename = "audioNormalizationSettings")]
    pub r#audio_normalization_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioNormalizationSettings>>,
    /// The name of the audio selector used as the source for this AudioDescription.
    #[builder(into)]
    #[serde(rename = "audioSelectorName")]
    pub r#audio_selector_name: String,
    /// Applies only if audioTypeControl is useConfigured. The values for audioType are defined in ISO-IEC 13818-1.
    #[builder(into)]
    #[serde(rename = "audioType")]
    pub r#audio_type: Option<String>,
    /// Determined how audio type is determined.
    #[builder(into)]
    #[serde(rename = "audioTypeControl")]
    pub r#audio_type_control: Option<String>,
    /// Settings to configure one or more solutions that insert audio watermarks in the audio encode. See Audio Watermark Settings for more details.
    #[builder(into)]
    #[serde(rename = "audioWatermarkSettings")]
    pub r#audio_watermark_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettings>>,
    /// Audio codec settings. See Audio Codec Settings for more details.
    #[builder(into)]
    #[serde(rename = "codecSettings")]
    pub r#codec_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionCodecSettings>>,
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Option<String>,
    #[builder(into)]
    #[serde(rename = "languageCodeControl")]
    pub r#language_code_control: Option<String>,
    /// The name of this audio description.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "remixSettings")]
    pub r#remix_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettings>>,
    /// Stream name RTMP destinations (URLs of type rtmp://)
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescription {
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
                    "audio_normalization_settings",
                    &self.r#audio_normalization_settings,
                ),
                to_pulumi_object_field(
                    "audio_selector_name",
                    &self.r#audio_selector_name,
                ),
                to_pulumi_object_field(
                    "audio_type",
                    &self.r#audio_type,
                ),
                to_pulumi_object_field(
                    "audio_type_control",
                    &self.r#audio_type_control,
                ),
                to_pulumi_object_field(
                    "audio_watermark_settings",
                    &self.r#audio_watermark_settings,
                ),
                to_pulumi_object_field(
                    "codec_settings",
                    &self.r#codec_settings,
                ),
                to_pulumi_object_field(
                    "language_code",
                    &self.r#language_code,
                ),
                to_pulumi_object_field(
                    "language_code_control",
                    &self.r#language_code_control,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "remix_settings",
                    &self.r#remix_settings,
                ),
                to_pulumi_object_field(
                    "stream_name",
                    &self.r#stream_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescription {
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
                    r#audio_normalization_settings: {
                        let field_value = match fields_map.get("audio_normalization_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_normalization_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_selector_name: {
                        let field_value = match fields_map.get("audio_selector_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_selector_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_type: {
                        let field_value = match fields_map.get("audio_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_type_control: {
                        let field_value = match fields_map.get("audio_type_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_type_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_watermark_settings: {
                        let field_value = match fields_map.get("audio_watermark_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_watermark_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codec_settings: {
                        let field_value = match fields_map.get("codec_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'codec_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("language_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#language_code_control: {
                        let field_value = match fields_map.get("language_code_control") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remix_settings: {
                        let field_value = match fields_map.get("remix_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'remix_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_name: {
                        let field_value = match fields_map.get("stream_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
