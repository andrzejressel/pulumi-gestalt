#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification {
    /// DTMF character that clears the accumulated DTMF digits and immediately ends the input.
    #[builder(into)]
    #[serde(rename = "deletionCharacter")]
    pub r#deletion_character: String,
    /// DTMF character that immediately ends input. If the user does not press this character, the input ends after the end timeout.
    #[builder(into)]
    #[serde(rename = "endCharacter")]
    pub r#end_character: String,
    /// How long the bot should wait after the last DTMF character input before assuming that the input has concluded.
    #[builder(into)]
    #[serde(rename = "endTimeoutMs")]
    pub r#end_timeout_ms: i32,
    /// Maximum number of DTMF digits allowed in an utterance.
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification {
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
                "deletion_character".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deletion_character,
                )
                .await,
            );
            map.insert(
                "end_character".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_character,
                )
                .await,
            );
            map.insert(
                "end_timeout_ms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_timeout_ms,
                )
                .await,
            );
            map.insert(
                "max_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_length,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification {
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
                    r#deletion_character: {
                        let field_value = match fields_map.get("deletion_character") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletion_character' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_character: {
                        let field_value = match fields_map.get("end_character") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_character' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_timeout_ms: {
                        let field_value = match fields_map.get("end_timeout_ms") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_timeout_ms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_length: {
                        let field_value = match fields_map.get("max_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
