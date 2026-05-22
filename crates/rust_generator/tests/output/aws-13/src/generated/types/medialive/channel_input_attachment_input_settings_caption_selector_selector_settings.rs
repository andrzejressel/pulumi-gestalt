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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "ancillarySourceSettings",
                    &self.r#ancillary_source_settings,
                ),
                to_pulumi_object_field(
                    "aribSourceSettings",
                    &self.r#arib_source_settings,
                ),
                to_pulumi_object_field(
                    "dvbSubSourceSettings",
                    &self.r#dvb_sub_source_settings,
                ),
                to_pulumi_object_field(
                    "embeddedSourceSettings",
                    &self.r#embedded_source_settings,
                ),
                to_pulumi_object_field(
                    "scte20SourceSettings",
                    &self.r#scte_20_source_settings,
                ),
                to_pulumi_object_field(
                    "scte27SourceSettings",
                    &self.r#scte_27_source_settings,
                ),
                to_pulumi_object_field(
                    "teletextSourceSettings",
                    &self.r#teletext_source_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettings {
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
                    r#ancillary_source_settings: {
                        let field_value = match fields_map.get("ancillarySourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ancillarySourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arib_source_settings: {
                        let field_value = match fields_map.get("aribSourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'aribSourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_source_settings: {
                        let field_value = match fields_map.get("dvbSubSourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbSubSourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedded_source_settings: {
                        let field_value = match fields_map.get("embeddedSourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embeddedSourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_20_source_settings: {
                        let field_value = match fields_map.get("scte20SourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte20SourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_27_source_settings: {
                        let field_value = match fields_map.get("scte27SourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte27SourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#teletext_source_settings: {
                        let field_value = match fields_map.get("teletextSourceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'teletextSourceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
