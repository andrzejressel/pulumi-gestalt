#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings {
    /// Settings specific to the container type of the file. See Container Settings for more details.
    #[builder(into)]
    #[serde(rename = "containerSettings")]
    pub r#container_settings: Option<Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings>>,
    /// Output file extension.
    #[builder(into)]
    #[serde(rename = "extension")]
    pub r#extension: Option<String>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "container_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_settings,
                )
                .await,
            );
            map.insert(
                "extension".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extension,
                )
                .await,
            );
            map.insert(
                "name_modifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_modifier,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettings {
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
                    r#container_settings: {
                        let field_value = match fields_map.get("container_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extension: {
                        let field_value = match fields_map.get("extension") {
                            Some(value) => value,
                            None => bail!("Missing field 'extension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_modifier: {
                        let field_value = match fields_map.get("name_modifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_modifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
