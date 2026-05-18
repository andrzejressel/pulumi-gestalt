#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings {
    /// Specifies behavior of last resort when input video os lost.
    #[builder(into)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Option<String>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Frame")]
    pub r#timed_metadata_id_3_frame: Option<String>,
    #[builder(into)]
    #[serde(rename = "timedMetadataId3Period")]
    pub r#timed_metadata_id_3_period: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings {
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
                    "input_loss_action",
                    &self.r#input_loss_action,
                ),
                to_pulumi_object_field(
                    "timed_metadata_id_3_frame",
                    &self.r#timed_metadata_id_3_frame,
                ),
                to_pulumi_object_field(
                    "timed_metadata_id_3_period",
                    &self.r#timed_metadata_id_3_period,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings {
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
                    r#input_loss_action: {
                        let field_value = match fields_map.get("input_loss_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_loss_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_frame: {
                        let field_value = match fields_map.get("timed_metadata_id_3_frame") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_id_3_frame' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timed_metadata_id_3_period: {
                        let field_value = match fields_map.get("timed_metadata_id_3_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'timed_metadata_id_3_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
