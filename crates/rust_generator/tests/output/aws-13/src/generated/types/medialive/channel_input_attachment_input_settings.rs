#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettings {
    /// Used to select the audio stream to decode for inputs that have multiple. See Audio Selectors for more details.
    #[builder(into)]
    #[serde(rename = "audioSelectors")]
    pub r#audio_selectors: Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelector>>,
    /// Used to select the caption input to use for inputs that have multiple available. See Caption Selectors for more details.
    #[builder(into)]
    #[serde(rename = "captionSelectors")]
    pub r#caption_selectors: Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelector>>,
    /// Enable or disable the deblock filter when filtering.
    #[builder(into)]
    #[serde(rename = "deblockFilter")]
    pub r#deblock_filter: Option<String>,
    /// Enable or disable the denoise filter when filtering.
    #[builder(into)]
    #[serde(rename = "denoiseFilter")]
    pub r#denoise_filter: Option<String>,
    /// Adjusts the magnitude of filtering from 1 (minimal) to 5 (strongest).
    #[builder(into)]
    #[serde(rename = "filterStrength")]
    pub r#filter_strength: Option<i32>,
    /// Turns on the filter for the input.
    #[builder(into)]
    #[serde(rename = "inputFilter")]
    pub r#input_filter: Option<String>,
    /// Input settings. See Network Input Settings for more details.
    #[builder(into)]
    #[serde(rename = "networkInputSettings")]
    pub r#network_input_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsNetworkInputSettings>>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Option<i32>,
    /// Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in the input.
    #[builder(into)]
    #[serde(rename = "smpte2038DataPreference")]
    pub r#smpte_2038_data_preference: Option<String>,
    /// Loop input if it is a file.
    #[builder(into)]
    #[serde(rename = "sourceEndBehavior")]
    pub r#source_end_behavior: Option<String>,
    #[builder(into)]
    #[serde(rename = "videoSelector")]
    pub r#video_selector: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsVideoSelector>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentInputSettings {
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
                    "audio_selectors",
                    &self.r#audio_selectors,
                ),
                to_pulumi_object_field(
                    "caption_selectors",
                    &self.r#caption_selectors,
                ),
                to_pulumi_object_field(
                    "deblock_filter",
                    &self.r#deblock_filter,
                ),
                to_pulumi_object_field(
                    "denoise_filter",
                    &self.r#denoise_filter,
                ),
                to_pulumi_object_field(
                    "filter_strength",
                    &self.r#filter_strength,
                ),
                to_pulumi_object_field(
                    "input_filter",
                    &self.r#input_filter,
                ),
                to_pulumi_object_field(
                    "network_input_settings",
                    &self.r#network_input_settings,
                ),
                to_pulumi_object_field(
                    "scte_35_pid",
                    &self.r#scte_35_pid,
                ),
                to_pulumi_object_field(
                    "smpte_2038_data_preference",
                    &self.r#smpte_2038_data_preference,
                ),
                to_pulumi_object_field(
                    "source_end_behavior",
                    &self.r#source_end_behavior,
                ),
                to_pulumi_object_field(
                    "video_selector",
                    &self.r#video_selector,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettings {
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
                    r#audio_selectors: {
                        let field_value = match fields_map.get("audio_selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_selectors: {
                        let field_value = match fields_map.get("caption_selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deblock_filter: {
                        let field_value = match fields_map.get("deblock_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'deblock_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denoise_filter: {
                        let field_value = match fields_map.get("denoise_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'denoise_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_strength: {
                        let field_value = match fields_map.get("filter_strength") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_strength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_filter: {
                        let field_value = match fields_map.get("input_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_input_settings: {
                        let field_value = match fields_map.get("network_input_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_input_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#smpte_2038_data_preference: {
                        let field_value = match fields_map.get("smpte_2038_data_preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'smpte_2038_data_preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_end_behavior: {
                        let field_value = match fields_map.get("source_end_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_end_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_selector: {
                        let field_value = match fields_map.get("video_selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
