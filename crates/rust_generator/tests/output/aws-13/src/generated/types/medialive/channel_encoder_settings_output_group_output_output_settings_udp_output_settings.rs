#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings {
    /// UDP output buffering in milliseconds.
    #[builder(into)]
    #[serde(rename = "bufferMsec")]
    pub r#buffer_msec: Option<i32>,
    /// UDP container settings. See Container Settings for more details.
    #[builder(into)]
    #[serde(rename = "containerSettings")]
    pub r#container_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings>,
    /// Destination address and port number for RTP or UDP packets. See Destination for more details.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination>,
    #[builder(into)]
    #[serde(rename = "fecOutputSettings")]
    pub r#fec_output_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("buffer_msec".to_string(), self.r#buffer_msec.to_pulumi_value().await);
            map.insert("container_settings".to_string(), self.r#container_settings.to_pulumi_value().await);
            map.insert("destination".to_string(), self.r#destination.to_pulumi_value().await);
            map.insert("fec_output_settings".to_string(), self.r#fec_output_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettings {
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
                    r#buffer_msec: {
                        let field_value = match fields_map.get("buffer_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffer_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#container_settings: {
                        let field_value = match fields_map.get("container_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettings> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsDestination> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#fec_output_settings: {
                        let field_value = match fields_map.get("fec_output_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'fec_output_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
