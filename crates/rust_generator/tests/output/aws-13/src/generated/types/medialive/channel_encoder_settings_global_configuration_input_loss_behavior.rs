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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "black_frame_msec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#black_frame_msec,
                )
                .await,
            );
            map.insert(
                "input_loss_image_color".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_loss_image_color,
                )
                .await,
            );
            map.insert(
                "input_loss_image_slate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_loss_image_slate,
                )
                .await,
            );
            map.insert(
                "input_loss_image_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_loss_image_type,
                )
                .await,
            );
            map.insert(
                "repeat_frame_msec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repeat_frame_msec,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
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
                    r#black_frame_msec: {
                        let field_value = match fields_map.get("black_frame_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'black_frame_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_color: {
                        let field_value = match fields_map.get("input_loss_image_color") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_slate: {
                        let field_value = match fields_map.get("input_loss_image_slate") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_slate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_loss_image_type: {
                        let field_value = match fields_map.get("input_loss_image_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_image_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repeat_frame_msec: {
                        let field_value = match fields_map.get("repeat_frame_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'repeat_frame_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
