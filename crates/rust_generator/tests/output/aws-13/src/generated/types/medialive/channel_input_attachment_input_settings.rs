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
                    "audioSelectors",
                    &self.r#audio_selectors,
                ),
                to_pulumi_object_field(
                    "captionSelectors",
                    &self.r#caption_selectors,
                ),
                to_pulumi_object_field(
                    "deblockFilter",
                    &self.r#deblock_filter,
                ),
                to_pulumi_object_field(
                    "denoiseFilter",
                    &self.r#denoise_filter,
                ),
                to_pulumi_object_field(
                    "filterStrength",
                    &self.r#filter_strength,
                ),
                to_pulumi_object_field(
                    "inputFilter",
                    &self.r#input_filter,
                ),
                to_pulumi_object_field(
                    "networkInputSettings",
                    &self.r#network_input_settings,
                ),
                to_pulumi_object_field(
                    "scte35Pid",
                    &self.r#scte_35_pid,
                ),
                to_pulumi_object_field(
                    "smpte2038DataPreference",
                    &self.r#smpte_2038_data_preference,
                ),
                to_pulumi_object_field(
                    "sourceEndBehavior",
                    &self.r#source_end_behavior,
                ),
                to_pulumi_object_field(
                    "videoSelector",
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
                        let field_value = match fields_map.get("audioSelectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioSelectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_selectors: {
                        let field_value = match fields_map.get("captionSelectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'captionSelectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deblock_filter: {
                        let field_value = match fields_map.get("deblockFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'deblockFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denoise_filter: {
                        let field_value = match fields_map.get("denoiseFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'denoiseFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_strength: {
                        let field_value = match fields_map.get("filterStrength") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterStrength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_filter: {
                        let field_value = match fields_map.get("inputFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_input_settings: {
                        let field_value = match fields_map.get("networkInputSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInputSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#smpte_2038_data_preference: {
                        let field_value = match fields_map.get("smpte2038DataPreference") {
                            Some(value) => value,
                            None => bail!("Missing field 'smpte2038DataPreference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_end_behavior: {
                        let field_value = match fields_map.get("sourceEndBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceEndBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_selector: {
                        let field_value = match fields_map.get("videoSelector") {
                            Some(value) => value,
                            None => bail!("Missing field 'videoSelector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
