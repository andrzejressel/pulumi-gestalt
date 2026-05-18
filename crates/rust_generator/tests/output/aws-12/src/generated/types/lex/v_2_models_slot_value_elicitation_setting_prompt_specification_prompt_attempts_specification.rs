#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification {
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    #[builder(into)]
    #[serde(rename = "allowedInputTypes")]
    pub r#allowed_input_types: Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes>,
    #[builder(into)]
    #[serde(rename = "audioAndDtmfInputSpecification")]
    pub r#audio_and_dtmf_input_specification: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification>>,
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    #[builder(into)]
    #[serde(rename = "textInputSpecification")]
    pub r#text_input_specification: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationTextInputSpecification>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification {
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
                    "allow_interrupt",
                    &self.r#allow_interrupt,
                ),
                to_pulumi_object_field(
                    "allowed_input_types",
                    &self.r#allowed_input_types,
                ),
                to_pulumi_object_field(
                    "audio_and_dtmf_input_specification",
                    &self.r#audio_and_dtmf_input_specification,
                ),
                to_pulumi_object_field(
                    "map_block_key",
                    &self.r#map_block_key,
                ),
                to_pulumi_object_field(
                    "text_input_specification",
                    &self.r#text_input_specification,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification {
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
                    r#allow_interrupt: {
                        let field_value = match fields_map.get("allow_interrupt") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_interrupt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_input_types: {
                        let field_value = match fields_map.get("allowed_input_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_input_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_and_dtmf_input_specification: {
                        let field_value = match fields_map.get("audio_and_dtmf_input_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_and_dtmf_input_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_block_key: {
                        let field_value = match fields_map.get("map_block_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_block_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_input_specification: {
                        let field_value = match fields_map.get("text_input_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_input_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
