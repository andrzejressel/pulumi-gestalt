#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxAgentAdvancedSettings {
    /// If present, incoming audio is exported by Dialogflow to the configured Google Cloud Storage destination. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "audioExportGcsDestination")]
    pub r#audio_export_gcs_destination: Option<Box<super::super::types::diagflow::CxAgentAdvancedSettingsAudioExportGcsDestination>>,
    /// Define behaviors for DTMF (dual tone multi frequency). DTMF settings does not override each other. DTMF settings set at different levels define DTMF detections running in parallel. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// * Page level
    /// * Parameter level
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dtmfSettings")]
    pub r#dtmf_settings: Option<Box<super::super::types::diagflow::CxAgentAdvancedSettingsDtmfSettings>>,
    /// Settings for logging. Settings for Dialogflow History, Contact Center messages, StackDriver logs, and speech logging. Exposed at the following levels:
    /// * Agent level
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "loggingSettings")]
    pub r#logging_settings: Option<Box<super::super::types::diagflow::CxAgentAdvancedSettingsLoggingSettings>>,
    /// Settings for speech to text detection. Exposed at the following levels:
    /// * Agent level
    /// * Flow level
    /// * Page level
    /// * Parameter level
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "speechSettings")]
    pub r#speech_settings: Option<Box<super::super::types::diagflow::CxAgentAdvancedSettingsSpeechSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxAgentAdvancedSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "audio_export_gcs_destination",
                    &self.r#audio_export_gcs_destination,
                ),
                to_pulumi_object_field(
                    "dtmf_settings",
                    &self.r#dtmf_settings,
                ),
                to_pulumi_object_field(
                    "logging_settings",
                    &self.r#logging_settings,
                ),
                to_pulumi_object_field(
                    "speech_settings",
                    &self.r#speech_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxAgentAdvancedSettings {
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
                    r#audio_export_gcs_destination: {
                        let field_value = match fields_map.get("audio_export_gcs_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'audio_export_gcs_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dtmf_settings: {
                        let field_value = match fields_map.get("dtmf_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'dtmf_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging_settings: {
                        let field_value = match fields_map.get("logging_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#speech_settings: {
                        let field_value = match fields_map.get("speech_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'speech_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
