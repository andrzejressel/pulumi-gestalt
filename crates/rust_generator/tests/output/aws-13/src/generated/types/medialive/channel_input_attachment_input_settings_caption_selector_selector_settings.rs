#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings {
    /// Ancillary Source Settings. See Ancillary Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "ancillarySourceSettings")]
    pub r#ancillary_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings>>,
    /// ARIB Source Settings.
    #[builder(into)]
    #[serde(rename = "aribSourceSettings")]
    pub r#arib_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings>>,
    /// DVB Sub Source Settings. See DVB Sub Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "dvbSubSourceSettings")]
    pub r#dvb_sub_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings>>,
    /// Embedded Source Settings. See Embedded Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "embeddedSourceSettings")]
    pub r#embedded_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings>>,
    /// SCTE20 Source Settings. See SCTE 20 Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "scte20SourceSettings")]
    pub r#scte_20_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings>>,
    /// SCTE27 Source Settings. See SCTE 27 Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "scte27SourceSettings")]
    pub r#scte_27_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings>>,
    /// Teletext Source Settings. See Teletext Source Settings for more details.
    #[builder(into)]
    #[serde(rename = "teletextSourceSettings")]
    pub r#teletext_source_settings: Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ancillary_source_settings".to_string(), self.r#ancillary_source_settings.to_pulumi_value().await);
            map.insert("arib_source_settings".to_string(), self.r#arib_source_settings.to_pulumi_value().await);
            map.insert("dvb_sub_source_settings".to_string(), self.r#dvb_sub_source_settings.to_pulumi_value().await);
            map.insert("embedded_source_settings".to_string(), self.r#embedded_source_settings.to_pulumi_value().await);
            map.insert("scte_20_source_settings".to_string(), self.r#scte_20_source_settings.to_pulumi_value().await);
            map.insert("scte_27_source_settings".to_string(), self.r#scte_27_source_settings.to_pulumi_value().await);
            map.insert("teletext_source_settings".to_string(), self.r#teletext_source_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings {
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
                    r#ancillary_source_settings: {
                        let field_value = match fields_map.get("ancillary_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ancillary_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAncillarySourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#arib_source_settings: {
                        let field_value = match fields_map.get("arib_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'arib_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsAribSourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_source_settings: {
                        let field_value = match fields_map.get("dvb_sub_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_sub_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsDvbSubSourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#embedded_source_settings: {
                        let field_value = match fields_map.get("embedded_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embedded_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scte_20_source_settings: {
                        let field_value = match fields_map.get("scte_20_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_20_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte20SourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scte_27_source_settings: {
                        let field_value = match fields_map.get("scte_27_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_27_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsScte27SourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#teletext_source_settings: {
                        let field_value = match fields_map.get("teletext_source_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'teletext_source_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsTeletextSourceSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
