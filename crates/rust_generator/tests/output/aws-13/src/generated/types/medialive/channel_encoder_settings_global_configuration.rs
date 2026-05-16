#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsGlobalConfiguration {
    /// Value to set the initial audio gain for the Live Event.
    #[builder(into)]
    #[serde(rename = "initialAudioGain")]
    pub r#initial_audio_gain: Option<i32>,
    /// Indicates the action to take when the current input completes (e.g. end-of-file). When switchAndLoopInputs is configured the encoder will restart at the beginning of the first input. When “none” is configured the encoder will transcode either black, a solid color, or a user specified slate images per the “Input Loss Behavior” configuration until the next input switch occurs (which is controlled through the Channel Schedule API).
    #[builder(into)]
    #[serde(rename = "inputEndAction")]
    pub r#input_end_action: Option<String>,
    /// Settings for system actions when input is lost. See Input Loss Behavior for more details.
    #[builder(into)]
    #[serde(rename = "inputLossBehavior")]
    pub r#input_loss_behavior: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehavior>>,
    /// Indicates how MediaLive pipelines are synchronized. PIPELINE\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the other. EPOCH\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the Unix epoch.
    #[builder(into)]
    #[serde(rename = "outputLockingMode")]
    pub r#output_locking_mode: Option<String>,
    /// Indicates whether the rate of frames emitted by the Live encoder should be paced by its system clock (which optionally may be locked to another source via NTP) or should be locked to the clock of the source that is providing the input stream.
    #[builder(into)]
    #[serde(rename = "outputTimingSource")]
    pub r#output_timing_source: Option<String>,
    /// Adjusts video input buffer for streams with very low video framerates. This is commonly set to enabled for music channels with less than one video frame per second.
    #[builder(into)]
    #[serde(rename = "supportLowFramerateInputs")]
    pub r#support_low_framerate_inputs: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsGlobalConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("initial_audio_gain".to_string(), self.r#initial_audio_gain.to_pulumi_value().await);
            map.insert("input_end_action".to_string(), self.r#input_end_action.to_pulumi_value().await);
            map.insert("input_loss_behavior".to_string(), self.r#input_loss_behavior.to_pulumi_value().await);
            map.insert("output_locking_mode".to_string(), self.r#output_locking_mode.to_pulumi_value().await);
            map.insert("output_timing_source".to_string(), self.r#output_timing_source.to_pulumi_value().await);
            map.insert("support_low_framerate_inputs".to_string(), self.r#support_low_framerate_inputs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsGlobalConfiguration {
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
                    r#initial_audio_gain: {
                        let field_value = match fields_map.get("initial_audio_gain") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_audio_gain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_end_action: {
                        let field_value = match fields_map.get("input_end_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_end_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_behavior: {
                        let field_value = match fields_map.get("input_loss_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehavior>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_locking_mode: {
                        let field_value = match fields_map.get("output_locking_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_locking_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_timing_source: {
                        let field_value = match fields_map.get("output_timing_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_timing_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#support_low_framerate_inputs: {
                        let field_value = match fields_map.get("support_low_framerate_inputs") {
                            Some(value) => value,
                            None => bail!("Missing field 'support_low_framerate_inputs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
