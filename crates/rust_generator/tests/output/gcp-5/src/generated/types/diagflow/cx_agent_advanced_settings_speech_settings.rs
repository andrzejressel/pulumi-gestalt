#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxAgentAdvancedSettingsSpeechSettings {
    /// Sensitivity of the speech model that detects the end of speech. Scale from 0 to 100.
    #[builder(into)]
    #[serde(rename = "endpointerSensitivity")]
    pub r#endpointer_sensitivity: Option<i32>,
    /// Mapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).
    /// An object containing a list of **"key": value** pairs. Example: **{ "name": "wrench", "mass": "1.3kg", "count": "3" }**.
    #[builder(into)]
    #[serde(rename = "models")]
    pub r#models: Option<std::collections::HashMap<String, String>>,
    /// Timeout before detecting no speech.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "noSpeechTimeout")]
    pub r#no_speech_timeout: Option<String>,
    /// Use timeout based endpointing, interpreting endpointer sensitivy as seconds of timeout value.
    #[builder(into)]
    #[serde(rename = "useTimeoutBasedEndpointing")]
    pub r#use_timeout_based_endpointing: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxAgentAdvancedSettingsSpeechSettings {
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
                "endpointer_sensitivity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpointer_sensitivity,
                )
                .await,
            );
            map.insert(
                "models".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#models,
                )
                .await,
            );
            map.insert(
                "no_speech_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_speech_timeout,
                )
                .await,
            );
            map.insert(
                "use_timeout_based_endpointing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_timeout_based_endpointing,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxAgentAdvancedSettingsSpeechSettings {
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
                    r#endpointer_sensitivity: {
                        let field_value = match fields_map.get("endpointer_sensitivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpointer_sensitivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#models: {
                        let field_value = match fields_map.get("models") {
                            Some(value) => value,
                            None => bail!("Missing field 'models' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_speech_timeout: {
                        let field_value = match fields_map.get("no_speech_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_speech_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_timeout_based_endpointing: {
                        let field_value = match fields_map.get("use_timeout_based_endpointing") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_timeout_based_endpointing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
