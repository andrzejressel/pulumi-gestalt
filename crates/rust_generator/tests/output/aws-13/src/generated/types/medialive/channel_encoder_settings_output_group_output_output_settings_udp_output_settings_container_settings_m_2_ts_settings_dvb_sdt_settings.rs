#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings {
    #[builder(into)]
    #[serde(rename = "outputSdt")]
    pub r#output_sdt: Option<String>,
    #[builder(into)]
    #[serde(rename = "repInterval")]
    pub r#rep_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "serviceProviderName")]
    pub r#service_provider_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("output_sdt".to_string(), self.r#output_sdt.to_pulumi_value().await);
            map.insert("rep_interval".to_string(), self.r#rep_interval.to_pulumi_value().await);
            map.insert("service_name".to_string(), self.r#service_name.to_pulumi_value().await);
            map.insert("service_provider_name".to_string(), self.r#service_provider_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsContainerSettingsM2TsSettingsDvbSdtSettings {
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
                    r#output_sdt: {
                        let field_value = match fields_map.get("output_sdt") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_sdt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rep_interval: {
                        let field_value = match fields_map.get("rep_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'rep_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_name: {
                        let field_value = match fields_map.get("service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_provider_name: {
                        let field_value = match fields_map.get("service_provider_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_provider_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
