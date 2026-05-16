#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiplexProgramMultiplexProgramSettings {
    /// Enum for preferred channel pipeline. Options are `CURRENTLY_ACTIVE`, `PIPELINE_0`, or `PIPELINE_1`.
    #[builder(into)]
    #[serde(rename = "preferredChannelPipeline")]
    pub r#preferred_channel_pipeline: String,
    /// Unique program number.
    #[builder(into)]
    #[serde(rename = "programNumber")]
    pub r#program_number: i32,
    /// Service Descriptor. See Service Descriptor for more details.
    #[builder(into)]
    #[serde(rename = "serviceDescriptor")]
    pub r#service_descriptor: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsServiceDescriptor>>,
    /// Video settings. See Video Settings for more details.
    #[builder(into)]
    #[serde(rename = "videoSettings")]
    pub r#video_settings: Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MultiplexProgramMultiplexProgramSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("preferred_channel_pipeline".to_string(), self.r#preferred_channel_pipeline.to_pulumi_value().await);
            map.insert("program_number".to_string(), self.r#program_number.to_pulumi_value().await);
            map.insert("service_descriptor".to_string(), self.r#service_descriptor.to_pulumi_value().await);
            map.insert("video_settings".to_string(), self.r#video_settings.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MultiplexProgramMultiplexProgramSettings {
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
                    r#preferred_channel_pipeline: {
                        let field_value = match fields_map.get("preferred_channel_pipeline") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_channel_pipeline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#program_number: {
                        let field_value = match fields_map.get("program_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'program_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_descriptor: {
                        let field_value = match fields_map.get("service_descriptor") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_descriptor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsServiceDescriptor>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#video_settings: {
                        let field_value = match fields_map.get("video_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'video_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettings>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
