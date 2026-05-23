#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsGlobalConfiguration {
    /// Value to set the initial audio gain for the Live Event.
    #[builder(into)]
    pub r#initial_audio_gain: Option<i32>,
    /// Indicates the action to take when the current input completes (e.g. end-of-file). When switchAndLoopInputs is configured the encoder will restart at the beginning of the first input. When “none” is configured the encoder will transcode either black, a solid color, or a user specified slate images per the “Input Loss Behavior” configuration until the next input switch occurs (which is controlled through the Channel Schedule API).
    #[builder(into)]
    pub r#input_end_action: Option<String>,
    /// Settings for system actions when input is lost. See Input Loss Behavior for more details.
    #[builder(into)]
    pub r#input_loss_behavior: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehavior>>,
    /// Indicates how MediaLive pipelines are synchronized. PIPELINE\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the other. EPOCH\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the Unix epoch.
    #[builder(into)]
    pub r#output_locking_mode: Option<String>,
    /// Indicates whether the rate of frames emitted by the Live encoder should be paced by its system clock (which optionally may be locked to another source via NTP) or should be locked to the clock of the source that is providing the input stream.
    #[builder(into)]
    pub r#output_timing_source: Option<String>,
    /// Adjusts video input buffer for streams with very low video framerates. This is commonly set to enabled for music channels with less than one video frame per second.
    #[builder(into)]
    pub r#support_low_framerate_inputs: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsGlobalConfiguration {
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
                    "initialAudioGain",
                    &self.r#initial_audio_gain,
                ),
                to_pulumi_object_field(
                    "inputEndAction",
                    &self.r#input_end_action,
                ),
                to_pulumi_object_field(
                    "inputLossBehavior",
                    &self.r#input_loss_behavior,
                ),
                to_pulumi_object_field(
                    "outputLockingMode",
                    &self.r#output_locking_mode,
                ),
                to_pulumi_object_field(
                    "outputTimingSource",
                    &self.r#output_timing_source,
                ),
                to_pulumi_object_field(
                    "supportLowFramerateInputs",
                    &self.r#support_low_framerate_inputs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsGlobalConfiguration {
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
                    r#initial_audio_gain: {
                        let field_value = match fields_map.get("initialAudioGain") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialAudioGain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_end_action: {
                        let field_value = match fields_map.get("inputEndAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputEndAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_behavior: {
                        let field_value = match fields_map.get("inputLossBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputLossBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_locking_mode: {
                        let field_value = match fields_map.get("outputLockingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputLockingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_timing_source: {
                        let field_value = match fields_map.get("outputTimingSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputTimingSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#support_low_framerate_inputs: {
                        let field_value = match fields_map.get("supportLowFramerateInputs") {
                            Some(value) => value,
                            None => bail!("Missing field 'supportLowFramerateInputs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
