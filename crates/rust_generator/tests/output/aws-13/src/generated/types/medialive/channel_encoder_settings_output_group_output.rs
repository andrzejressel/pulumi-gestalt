#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutput {
    /// The names of the audio descriptions used as audio sources for the output.
    #[builder(into)]
    pub r#audio_description_names: Option<Vec<String>>,
    /// The names of the caption descriptions used as caption sources for the output.
    #[builder(into)]
    pub r#caption_description_names: Option<Vec<String>>,
    /// The name used to identify an output.
    #[builder(into)]
    pub r#output_name: Option<String>,
    /// Settings for output. See Output Settings for more details.
    #[builder(into)]
    pub r#output_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettings>,
    /// The name of the video description used as video source for the output.
    #[builder(into)]
    pub r#video_description_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutput {
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
                    "audioDescriptionNames",
                    &self.r#audio_description_names,
                ),
                to_pulumi_object_field(
                    "captionDescriptionNames",
                    &self.r#caption_description_names,
                ),
                to_pulumi_object_field(
                    "outputName",
                    &self.r#output_name,
                ),
                to_pulumi_object_field(
                    "outputSettings",
                    &self.r#output_settings,
                ),
                to_pulumi_object_field(
                    "videoDescriptionName",
                    &self.r#video_description_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutput {
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
                    r#audio_description_names: {
                        let field_value = match fields_map.get("audioDescriptionNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioDescriptionNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caption_description_names: {
                        let field_value = match fields_map.get("captionDescriptionNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'captionDescriptionNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_name: {
                        let field_value = match fields_map.get("outputName") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_settings: {
                        let field_value = match fields_map.get("outputSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#video_description_name: {
                        let field_value = match fields_map.get("videoDescriptionName") {
                            Some(value) => value,
                            None => bail!("Missing field 'videoDescriptionName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
