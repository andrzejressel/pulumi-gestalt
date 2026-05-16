#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
    #[builder(into)]
    #[serde(rename = "blackFrameMsec")]
    pub r#black_frame_msec: Option<i32>,
    #[builder(into)]
    #[serde(rename = "inputLossImageColor")]
    pub r#input_loss_image_color: Option<String>,
    #[builder(into)]
    #[serde(rename = "inputLossImageSlate")]
    pub r#input_loss_image_slate: Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate>>,
    #[builder(into)]
    #[serde(rename = "inputLossImageType")]
    pub r#input_loss_image_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "repeatFrameMsec")]
    pub r#repeat_frame_msec: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("black_frame_msec".to_string(), self.r#black_frame_msec.to_pulumi_value().await);
            map.insert("input_loss_image_color".to_string(), self.r#input_loss_image_color.to_pulumi_value().await);
            map.insert("input_loss_image_slate".to_string(), self.r#input_loss_image_slate.to_pulumi_value().await);
            map.insert("input_loss_image_type".to_string(), self.r#input_loss_image_type.to_pulumi_value().await);
            map.insert("repeat_frame_msec".to_string(), self.r#repeat_frame_msec.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
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
                    r#black_frame_msec: {
                        let field_value = match fields_map.get("black_frame_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'black_frame_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_color: {
                        let field_value = match fields_map.get("input_loss_image_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_slate: {
                        let field_value = match fields_map.get("input_loss_image_slate") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_slate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_type: {
                        let field_value = match fields_map.get("input_loss_image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#repeat_frame_msec: {
                        let field_value = match fields_map.get("repeat_frame_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'repeat_frame_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
