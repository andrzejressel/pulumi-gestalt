#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescription {
    /// Indicates whether the caption track implements accessibility features such as written descriptions of spoken dialog, music, and sounds.
    #[builder(into)]
    #[serde(rename = "accessibility")]
    pub r#accessibility: Option<String>,
    /// Specifies which input caption selector to use as a caption source when generating output captions. This field should match a captionSelector name.
    #[builder(into)]
    #[serde(rename = "captionSelectorName")]
    pub r#caption_selector_name: String,
    /// Additional settings for captions destination that depend on the destination type. See Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "destinationSettings")]
    pub r#destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettings>>,
    /// ISO 639-2 three-digit code.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Option<String>,
    /// Human readable information to indicate captions available for players (eg. English, or Spanish).
    #[builder(into)]
    #[serde(rename = "languageDescription")]
    pub r#language_description: Option<String>,
    /// Name of the caption description. Used to associate a caption description with an output. Names must be unique within an event.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsCaptionDescription {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("accessibility".to_string(), self.r#accessibility.to_pulumi_value().await);
            map.insert("caption_selector_name".to_string(), self.r#caption_selector_name.to_pulumi_value().await);
            map.insert("destination_settings".to_string(), self.r#destination_settings.to_pulumi_value().await);
            map.insert("language_code".to_string(), self.r#language_code.to_pulumi_value().await);
            map.insert("language_description".to_string(), self.r#language_description.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsCaptionDescription {
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
                    r#accessibility: {
                        let field_value = match fields_map.get("accessibility") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessibility' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#caption_selector_name: {
                        let field_value = match fields_map.get("caption_selector_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'caption_selector_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination_settings: {
                        let field_value = match fields_map.get("destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#language_code: {
                        let field_value = match fields_map.get("language_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#language_description: {
                        let field_value = match fields_map.get("language_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'language_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
