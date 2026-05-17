#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsMotionGraphicsConfiguration {
    /// Motion Graphics Insertion.
    #[builder(into)]
    #[serde(rename = "motionGraphicsInsertion")]
    pub r#motion_graphics_insertion: Option<String>,
    /// Motion Graphics Settings. See Motion Graphics Settings for more details.
    #[builder(into)]
    #[serde(rename = "motionGraphicsSettings")]
    pub r#motion_graphics_settings: Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsMotionGraphicsConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "motion_graphics_insertion".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#motion_graphics_insertion,
                )
                .await,
            );
            map.insert(
                "motion_graphics_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#motion_graphics_settings,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsMotionGraphicsConfiguration {
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
                    r#motion_graphics_insertion: {
                        let field_value = match fields_map.get("motion_graphics_insertion") {
                            Some(value) => value,
                            None => bail!("Missing field 'motion_graphics_insertion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#motion_graphics_settings: {
                        let field_value = match fields_map.get("motion_graphics_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'motion_graphics_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
