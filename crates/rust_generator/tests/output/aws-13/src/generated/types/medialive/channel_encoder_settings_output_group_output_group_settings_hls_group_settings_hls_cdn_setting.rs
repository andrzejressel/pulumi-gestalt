#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting {
    #[builder(into)]
    #[serde(rename = "hlsAkamaiSettings")]
    pub r#hls_akamai_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsBasicPutSettings")]
    pub r#hls_basic_put_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsMediaStoreSettings")]
    pub r#hls_media_store_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings>>,
    #[builder(into)]
    #[serde(rename = "hlsS3Settings")]
    pub r#hls_s_3_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings>>,
    #[builder(into)]
    #[serde(rename = "hlsWebdavSettings")]
    pub r#hls_webdav_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("hls_akamai_settings".to_string(), self.r#hls_akamai_settings.to_pulumi_value().await);
            map.insert("hls_basic_put_settings".to_string(), self.r#hls_basic_put_settings.to_pulumi_value().await);
            map.insert("hls_media_store_settings".to_string(), self.r#hls_media_store_settings.to_pulumi_value().await);
            map.insert("hls_s_3_settings".to_string(), self.r#hls_s_3_settings.to_pulumi_value().await);
            map.insert("hls_webdav_settings".to_string(), self.r#hls_webdav_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting {
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
                    r#hls_akamai_settings: {
                        let field_value = match fields_map.get("hls_akamai_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_akamai_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_basic_put_settings: {
                        let field_value = match fields_map.get("hls_basic_put_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_basic_put_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_media_store_settings: {
                        let field_value = match fields_map.get("hls_media_store_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_media_store_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_s_3_settings: {
                        let field_value = match fields_map.get("hls_s_3_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_s_3_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_webdav_settings: {
                        let field_value = match fields_map.get("hls_webdav_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_webdav_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
