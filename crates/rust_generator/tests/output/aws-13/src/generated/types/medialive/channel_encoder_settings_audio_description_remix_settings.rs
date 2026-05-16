#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsAudioDescriptionRemixSettings {
    #[builder(into)]
    #[serde(rename = "channelMappings")]
    pub r#channel_mappings: Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping>,
    #[builder(into)]
    #[serde(rename = "channelsIn")]
    pub r#channels_in: Option<i32>,
    #[builder(into)]
    #[serde(rename = "channelsOut")]
    pub r#channels_out: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsAudioDescriptionRemixSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("channel_mappings".to_string(), self.r#channel_mappings.to_pulumi_value().await);
            map.insert("channels_in".to_string(), self.r#channels_in.to_pulumi_value().await);
            map.insert("channels_out".to_string(), self.r#channels_out.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsAudioDescriptionRemixSettings {
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
                    r#channel_mappings: {
                        let field_value = match fields_map.get("channel_mappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel_mappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionRemixSettingsChannelMapping> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#channels_in: {
                        let field_value = match fields_map.get("channels_in") {
                            Some(value) => value,
                            None => bail!("Missing field 'channels_in' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#channels_out: {
                        let field_value = match fields_map.get("channels_out") {
                            Some(value) => value,
                            None => bail!("Missing field 'channels_out' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
