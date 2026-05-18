#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxFlowAdvancedSettingsSpeechSettings {
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
    /// Use timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value.
    #[builder(into)]
    #[serde(rename = "useTimeoutBasedEndpointing")]
    pub r#use_timeout_based_endpointing: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxFlowAdvancedSettingsSpeechSettings {
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
                    "endpointer_sensitivity",
                    &self.r#endpointer_sensitivity,
                ),
                to_pulumi_object_field(
                    "models",
                    &self.r#models,
                ),
                to_pulumi_object_field(
                    "no_speech_timeout",
                    &self.r#no_speech_timeout,
                ),
                to_pulumi_object_field(
                    "use_timeout_based_endpointing",
                    &self.r#use_timeout_based_endpointing,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxFlowAdvancedSettingsSpeechSettings {
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
