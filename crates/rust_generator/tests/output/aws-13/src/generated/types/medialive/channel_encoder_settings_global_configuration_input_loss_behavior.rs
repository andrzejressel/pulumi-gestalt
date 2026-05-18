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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "black_frame_msec",
                    &self.r#black_frame_msec,
                ),
                to_pulumi_object_field(
                    "input_loss_image_color",
                    &self.r#input_loss_image_color,
                ),
                to_pulumi_object_field(
                    "input_loss_image_slate",
                    &self.r#input_loss_image_slate,
                ),
                to_pulumi_object_field(
                    "input_loss_image_type",
                    &self.r#input_loss_image_type,
                ),
                to_pulumi_object_field(
                    "repeat_frame_msec",
                    &self.r#repeat_frame_msec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
