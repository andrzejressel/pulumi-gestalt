#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
    /// ARIB Destination Settings.
    #[builder(into)]
    #[serde(rename = "aribDestinationSettings")]
    pub r#arib_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsAribDestinationSettings>>,
    /// Burn In Destination Settings. See Burn In Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "burnInDestinationSettings")]
    pub r#burn_in_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsBurnInDestinationSettings>>,
    /// DVB Sub Destination Settings. See DVB Sub Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "dvbSubDestinationSettings")]
    pub r#dvb_sub_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings>>,
    /// EBU TT D Destination Settings. See EBU TT D Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "ebuTtDDestinationSettings")]
    pub r#ebu_tt_d_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings>>,
    /// Embedded Destination Settings.
    #[builder(into)]
    #[serde(rename = "embeddedDestinationSettings")]
    pub r#embedded_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedDestinationSettings>>,
    /// Embedded Plus SCTE20 Destination Settings.
    #[builder(into)]
    #[serde(rename = "embeddedPlusScte20DestinationSettings")]
    pub r#embedded_plus_scte_20_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEmbeddedPlusScte20DestinationSettings>>,
    /// RTMP Caption Info Destination Settings.
    #[builder(into)]
    #[serde(rename = "rtmpCaptionInfoDestinationSettings")]
    pub r#rtmp_caption_info_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsRtmpCaptionInfoDestinationSettings>>,
    /// SCTE20 Plus Embedded Destination Settings.
    #[builder(into)]
    #[serde(rename = "scte20PlusEmbeddedDestinationSettings")]
    pub r#scte_20_plus_embedded_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte20PlusEmbeddedDestinationSettings>>,
    /// SCTE27 Destination Settings.
    #[builder(into)]
    #[serde(rename = "scte27DestinationSettings")]
    pub r#scte_27_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsScte27DestinationSettings>>,
    /// SMPTE TT Destination Settings.
    #[builder(into)]
    #[serde(rename = "smpteTtDestinationSettings")]
    pub r#smpte_tt_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsSmpteTtDestinationSettings>>,
    /// Teletext Destination Settings.
    #[builder(into)]
    #[serde(rename = "teletextDestinationSettings")]
    pub r#teletext_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTeletextDestinationSettings>>,
    /// TTML Destination Settings. See TTML Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "ttmlDestinationSettings")]
    pub r#ttml_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsTtmlDestinationSettings>>,
    /// WebVTT Destination Settings. See WebVTT Destination Settings for more details.
    #[builder(into)]
    #[serde(rename = "webvttDestinationSettings")]
    pub r#webvtt_destination_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsWebvttDestinationSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsCaptionDescriptionDestinationSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "arib_destination_settings",
                    &self.r#arib_destination_settings,
                ),
                to_pulumi_object_field(
                    "burn_in_destination_settings",
                    &self.r#burn_in_destination_settings,
                ),
                to_pulumi_object_field(
                    "dvb_sub_destination_settings",
                    &self.r#dvb_sub_destination_settings,
                ),
                to_pulumi_object_field(
                    "ebu_tt_d_destination_settings",
                    &self.r#ebu_tt_d_destination_settings,
                ),
                to_pulumi_object_field(
                    "embedded_destination_settings",
                    &self.r#embedded_destination_settings,
                ),
                to_pulumi_object_field(
                    "embedded_plus_scte_20_destination_settings",
                    &self.r#embedded_plus_scte_20_destination_settings,
                ),
                to_pulumi_object_field(
                    "rtmp_caption_info_destination_settings",
                    &self.r#rtmp_caption_info_destination_settings,
                ),
                to_pulumi_object_field(
                    "scte_20_plus_embedded_destination_settings",
                    &self.r#scte_20_plus_embedded_destination_settings,
                ),
                to_pulumi_object_field(
                    "scte_27_destination_settings",
                    &self.r#scte_27_destination_settings,
                ),
                to_pulumi_object_field(
                    "smpte_tt_destination_settings",
                    &self.r#smpte_tt_destination_settings,
                ),
                to_pulumi_object_field(
                    "teletext_destination_settings",
                    &self.r#teletext_destination_settings,
                ),
                to_pulumi_object_field(
                    "ttml_destination_settings",
                    &self.r#ttml_destination_settings,
                ),
                to_pulumi_object_field(
                    "webvtt_destination_settings",
                    &self.r#webvtt_destination_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("arib_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'arib_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#burn_in_destination_settings: {
                        let field_value = match fields_map.get("burn_in_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'burn_in_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dvb_sub_destination_settings: {
                        let field_value = match fields_map.get("dvb_sub_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dvb_sub_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ebu_tt_d_destination_settings: {
                        let field_value = match fields_map.get("ebu_tt_d_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebu_tt_d_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedded_destination_settings: {
                        let field_value = match fields_map.get("embedded_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embedded_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#embedded_plus_scte_20_destination_settings: {
                        let field_value = match fields_map.get("embedded_plus_scte_20_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'embedded_plus_scte_20_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rtmp_caption_info_destination_settings: {
                        let field_value = match fields_map.get("rtmp_caption_info_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'rtmp_caption_info_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_20_plus_embedded_destination_settings: {
                        let field_value = match fields_map.get("scte_20_plus_embedded_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_20_plus_embedded_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scte_27_destination_settings: {
                        let field_value = match fields_map.get("scte_27_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'scte_27_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smpte_tt_destination_settings: {
                        let field_value = match fields_map.get("smpte_tt_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'smpte_tt_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#teletext_destination_settings: {
                        let field_value = match fields_map.get("teletext_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'teletext_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttml_destination_settings: {
                        let field_value = match fields_map.get("ttml_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttml_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webvtt_destination_settings: {
                        let field_value = match fields_map.get("webvtt_destination_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'webvtt_destination_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
