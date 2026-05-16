#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutput {
    /// The names of the audio descriptions used as audio sources for the output.
    #[builder(into)]
    #[serde(rename = "audioDescriptionNames")]
    pub r#audio_description_names: Option<Vec<String>>,
    /// The names of the caption descriptions used as caption sources for the output.
    #[builder(into)]
    #[serde(rename = "captionDescriptionNames")]
    pub r#caption_description_names: Option<Vec<String>>,
    /// The name used to identify an output.
    #[builder(into)]
    #[serde(rename = "outputName")]
    pub r#output_name: Option<String>,
    /// Settings for output. See Output Settings for more details.
    #[builder(into)]
    #[serde(rename = "outputSettings")]
    pub r#output_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettings>,
    /// The name of the video description used as video source for the output.
    #[builder(into)]
    #[serde(rename = "videoDescriptionName")]
    pub r#video_description_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutput {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("audio_description_names".to_string(), self.r#audio_description_names.to_pulumi_value().await);
            map.insert("caption_description_names".to_string(), self.r#caption_description_names.to_pulumi_value().await);
            map.insert("output_name".to_string(), self.r#output_name.to_pulumi_value().await);
            map.insert("output_settings".to_string(), self.r#output_settings.to_pulumi_value().await);
            map.insert("video_description_name".to_string(), self.r#video_description_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutput {
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
                    r#audio_description_names: {
                        let field_value = match fields_map.get("audio_description_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_description_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caption_description_names: {
                        let field_value = match fields_map.get("caption_description_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_description_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_name: {
                        let field_value = match fields_map.get("output_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_settings: {
                        let field_value = match fields_map.get("output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettings> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#video_description_name: {
                        let field_value = match fields_map.get("video_description_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_description_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
