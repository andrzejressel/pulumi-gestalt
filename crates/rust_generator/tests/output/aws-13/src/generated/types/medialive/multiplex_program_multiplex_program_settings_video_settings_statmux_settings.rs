#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings {
    /// Maximum bitrate.
    #[builder(into)]
    #[serde(rename = "maximumBitrate")]
    pub r#maximum_bitrate: Option<i32>,
    /// Minimum bitrate.
    #[builder(into)]
    #[serde(rename = "minimumBitrate")]
    pub r#minimum_bitrate: Option<i32>,
    /// Priority value.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("maximum_bitrate".to_string(), self.r#maximum_bitrate.to_pulumi_value().await);
            map.insert("minimum_bitrate".to_string(), self.r#minimum_bitrate.to_pulumi_value().await);
            map.insert("priority".to_string(), self.r#priority.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings {
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
                    r#maximum_bitrate: {
                        let field_value = match fields_map.get("maximum_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#minimum_bitrate: {
                        let field_value = match fields_map.get("minimum_bitrate") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_bitrate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
