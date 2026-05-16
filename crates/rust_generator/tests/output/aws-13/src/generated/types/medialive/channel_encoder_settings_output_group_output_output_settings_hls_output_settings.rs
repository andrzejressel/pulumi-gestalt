#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings {
    #[builder(into)]
    #[serde(rename = "h265PackagingType")]
    pub r#h_265_packaging_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "hlsSettings")]
    pub r#hls_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Option<String>,
    #[builder(into)]
    #[serde(rename = "segmentModifier")]
    pub r#segment_modifier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("h_265_packaging_type".to_string(), self.r#h_265_packaging_type.to_pulumi_value().await);
            map.insert("hls_settings".to_string(), self.r#hls_settings.to_pulumi_value().await);
            map.insert("name_modifier".to_string(), self.r#name_modifier.to_pulumi_value().await);
            map.insert("segment_modifier".to_string(), self.r#segment_modifier.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings {
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
                    r#h_265_packaging_type: {
                        let field_value = match fields_map.get("h_265_packaging_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'h_265_packaging_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hls_settings: {
                        let field_value = match fields_map.get("hls_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'hls_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name_modifier: {
                        let field_value = match fields_map.get("name_modifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_modifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#segment_modifier: {
                        let field_value = match fields_map.get("segment_modifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment_modifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
