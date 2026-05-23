#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification {
    #[builder(into)]
    pub r#allow_interrupt: Option<bool>,
    #[builder(into)]
    pub r#allowed_input_types: Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAllowedInputTypes>,
    #[builder(into)]
    pub r#audio_and_dtmf_input_specification: Option<Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification>>,
    #[builder(into)]
    pub r#map_block_key: String,
    #[builder(into)]
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
                    "allowInterrupt",
                    &self.r#allow_interrupt,
                ),
                to_pulumi_object_field(
                    "allowedInputTypes",
                    &self.r#allowed_input_types,
                ),
                to_pulumi_object_field(
                    "audioAndDtmfInputSpecification",
                    &self.r#audio_and_dtmf_input_specification,
                ),
                to_pulumi_object_field(
                    "mapBlockKey",
                    &self.r#map_block_key,
                ),
                to_pulumi_object_field(
                    "textInputSpecification",
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
                        let field_value = match fields_map.get("allowInterrupt") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowInterrupt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_input_types: {
                        let field_value = match fields_map.get("allowedInputTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowedInputTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_and_dtmf_input_specification: {
                        let field_value = match fields_map.get("audioAndDtmfInputSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioAndDtmfInputSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_block_key: {
                        let field_value = match fields_map.get("mapBlockKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapBlockKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_input_specification: {
                        let field_value = match fields_map.get("textInputSpecification") {
                            Some(value) => value,
                            None => bail!("Missing field 'textInputSpecification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
