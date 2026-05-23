#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
    /// ARIB Destination Settings.
    #[builder(into)]
    pub r#arib_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings>>,
    /// Burn In Destination Settings. See Burn In Destination Settings for more details.
    #[builder(into)]
    pub r#burn_in_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings>>,
    /// DVB Sub Destination Settings. See DVB Sub Destination Settings for more details.
    #[builder(into)]
    pub r#dvb_sub_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings>>,
    /// EBU TT D Destination Settings. See EBU TT D Destination Settings for more details.
    #[builder(into)]
    pub r#ebu_tt_d_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings>>,
    /// Embedded Destination Settings.
    #[builder(into)]
    pub r#embedded_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings>>,
    /// Embedded Plus SCTE20 Destination Settings.
    #[builder(into)]
    pub r#embedded_plus_scte_20_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings>>,
    /// RTMP Caption Info Destination Settings.
    #[builder(into)]
    pub r#rtmp_caption_info_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings>>,
    /// SCTE20 Plus Embedded Destination Settings.
    #[builder(into)]
    pub r#scte_20_plus_embedded_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings>>,
    /// SCTE27 Destination Settings.
    #[builder(into)]
    pub r#scte_27_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings>>,
    /// SMPTE TT Destination Settings.
    #[builder(into)]
    pub r#smpte_tt_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings>>,
    /// Teletext Destination Settings.
    #[builder(into)]
    pub r#teletext_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings>>,
    /// TTML Destination Settings. See TTML Destination Settings for more details.
    #[builder(into)]
    pub r#ttml_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings>>,
    /// WebVTT Destination Settings. See WebVTT Destination Settings for more details.
    #[builder(into)]
    pub r#webvtt_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
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
                    "aribDestinationSettings",
                    &self.r#arib_destination_settings,
                ),
                to_pulumi_object_field(
                    "burnInDestinationSettings",
                    &self.r#burn_in_destination_settings,
                ),
                to_pulumi_object_field(
                    "dvbSubDestinationSettings",
                    &self.r#dvb_sub_destination_settings,
                ),
                to_pulumi_object_field(
                    "ebuTtDDestinationSettings",
                    &self.r#ebu_tt_d_destination_settings,
                ),
                to_pulumi_object_field(
                    "embeddedDestinationSettings",
                    &self.r#embedded_destination_settings,
                ),
                to_pulumi_object_field(
                    "embeddedPlusScte20DestinationSettings",
                    &self.r#embedded_plus_scte_20_destination_settings,
                ),
                to_pulumi_object_field(
                    "rtmpCaptionInfoDestinationSettings",
                    &self.r#rtmp_caption_info_destination_settings,
                ),
                to_pulumi_object_field(
                    "scte20PlusEmbeddedDestinationSettings",
                    &self.r#scte_20_plus_embedded_destination_settings,
                ),
                to_pulumi_object_field(
                    "scte27DestinationSettings",
                    &self.r#scte_27_destination_settings,
                ),
                to_pulumi_object_field(
                    "smpteTtDestinationSettings",
                    &self.r#smpte_tt_destination_settings,
                ),
                to_pulumi_object_field(
                    "teletextDestinationSettings",
                    &self.r#teletext_destination_settings,
                ),
                to_pulumi_object_field(
                    "ttmlDestinationSettings",
                    &self.r#ttml_destination_settings,
                ),
                to_pulumi_object_field(
                    "webvttDestinationSettings",
                    &self.r#webvtt_destination_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
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
                    r#arib_destination_settings: {
                        let field_value = match fields_map.get("aribDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'aribDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#burn_in_destination_settings: {
                        let field_value = match fields_map.get("burnInDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'burnInDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_destination_settings: {
                        let field_value = match fields_map.get("dvbSubDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvbSubDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebu_tt_d_destination_settings: {
                        let field_value = match fields_map.get("ebuTtDDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebuTtDDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedded_destination_settings: {
                        let field_value = match fields_map.get("embeddedDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embeddedDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedded_plus_scte_20_destination_settings: {
                        let field_value = match fields_map.get("embeddedPlusScte20DestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embeddedPlusScte20DestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rtmp_caption_info_destination_settings: {
                        let field_value = match fields_map.get("rtmpCaptionInfoDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rtmpCaptionInfoDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_20_plus_embedded_destination_settings: {
                        let field_value = match fields_map.get("scte20PlusEmbeddedDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte20PlusEmbeddedDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_27_destination_settings: {
                        let field_value = match fields_map.get("scte27DestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte27DestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smpte_tt_destination_settings: {
                        let field_value = match fields_map.get("smpteTtDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'smpteTtDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#teletext_destination_settings: {
                        let field_value = match fields_map.get("teletextDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'teletextDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttml_destination_settings: {
                        let field_value = match fields_map.get("ttmlDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttmlDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webvtt_destination_settings: {
                        let field_value = match fields_map.get("webvttDestinationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'webvttDestinationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
