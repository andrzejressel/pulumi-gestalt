#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxSecuritySettingsAudioExportSettings {
    /// Filename pattern for exported audio.
    #[builder(into)]
    #[serde(rename = "audioExportPattern")]
    pub r#audio_export_pattern: Option<String>,
    /// File format for exported audio file. Currently only in telephony recordings.
    /// * MULAW: G.711 mu-law PCM with 8kHz sample rate.
    /// * MP3: MP3 file format.
    /// * OGG: OGG Vorbis.
    /// Possible values are: `MULAW`, `MP3`, `OGG`.
    #[builder(into)]
    #[serde(rename = "audioFormat")]
    pub r#audio_format: Option<String>,
    /// Enable audio redaction if it is true.
    #[builder(into)]
    #[serde(rename = "enableAudioRedaction")]
    pub r#enable_audio_redaction: Option<bool>,
    /// Cloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy.
    #[builder(into)]
    #[serde(rename = "gcsBucket")]
    pub r#gcs_bucket: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxSecuritySettingsAudioExportSettings {
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
                    "audioExportPattern",
                    &self.r#audio_export_pattern,
                ),
                to_pulumi_object_field(
                    "audioFormat",
                    &self.r#audio_format,
                ),
                to_pulumi_object_field(
                    "enableAudioRedaction",
                    &self.r#enable_audio_redaction,
                ),
                to_pulumi_object_field(
                    "gcsBucket",
                    &self.r#gcs_bucket,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxSecuritySettingsAudioExportSettings {
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
                    r#audio_export_pattern: {
                        let field_value = match fields_map.get("audioExportPattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioExportPattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#audio_format: {
                        let field_value = match fields_map.get("audioFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'audioFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_audio_redaction: {
                        let field_value = match fields_map.get("enableAudioRedaction") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAudioRedaction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_bucket: {
                        let field_value = match fields_map.get("gcsBucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcsBucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
