#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettings {
    /// Constant bitrate value.
    #[builder(into)]
    #[serde(rename = "constantBitrate")]
    pub r#constant_bitrate: Option<i32>,
    /// Statmux settings. See Statmux Settings for more details.
    #[builder(into)]
    #[serde(rename = "statmuxSettings")]
    pub r#statmux_settings: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MultiplexProgramMultiplexProgramSettingsVideoSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("constant_bitrate".to_string(), self.r#constant_bitrate.to_pulumi_value().await);
            map.insert("statmux_settings".to_string(), self.r#statmux_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MultiplexProgramMultiplexProgramSettingsVideoSettings {
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
                    r#constant_bitrate: {
                        let field_value = match fields_map.get("constant_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'constant_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#statmux_settings: {
                        let field_value = match fields_map.get("statmux_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'statmux_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
