#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings {
    /// MediaLive will perform a failover if the specified audio selector is silent for the specified period. See Audio Silence Failover Settings for more details.
    #[builder(into)]
    #[serde(rename = "audioSilenceSettings")]
    pub r#audio_silence_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings>>,
    /// MediaLive will perform a failover if content is not detected in this input for the specified period. See Input Loss Failover Settings for more details.
    #[builder(into)]
    #[serde(rename = "inputLossSettings")]
    pub r#input_loss_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings>>,
    /// MediaLive will perform a failover if content is considered black for the specified period. See Video Black Failover Settings for more details.
    #[builder(into)]
    #[serde(rename = "videoBlackSettings")]
    pub r#video_black_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("audio_silence_settings".to_string(), self.r#audio_silence_settings.to_pulumi_value().await);
            map.insert("input_loss_settings".to_string(), self.r#input_loss_settings.to_pulumi_value().await);
            map.insert("video_black_settings".to_string(), self.r#video_black_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettings {
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
                    r#audio_silence_settings: {
                        let field_value = match fields_map.get("audio_silence_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_silence_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsAudioSilenceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_settings: {
                        let field_value = match fields_map.get("input_loss_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsInputLossSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#video_black_settings: {
                        let field_value = match fields_map.get("video_black_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_black_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverConditionFailoverConditionSettingsVideoBlackSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
