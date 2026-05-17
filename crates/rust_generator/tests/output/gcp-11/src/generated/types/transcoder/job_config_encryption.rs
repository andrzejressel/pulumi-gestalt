#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobConfigEncryption {
    /// Configuration for AES-128 encryption.
    #[builder(into)]
    #[serde(rename = "aes128")]
    pub r#aes_128: Option<Box<super::super::types::transcoder::JobConfigEncryptionAes128>>,
    /// DRM system(s) to use; at least one must be specified. If a DRM system is omitted, it is considered disabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "drmSystems")]
    pub r#drm_systems: Option<Box<super::super::types::transcoder::JobConfigEncryptionDrmSystems>>,
    /// Identifier for this set of encryption options.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Configuration for MPEG Common Encryption (MPEG-CENC).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mpegCenc")]
    pub r#mpeg_cenc: Option<Box<super::super::types::transcoder::JobConfigEncryptionMpegCenc>>,
    /// Configuration for SAMPLE-AES encryption.
    #[builder(into)]
    #[serde(rename = "sampleAes")]
    pub r#sample_aes: Option<Box<super::super::types::transcoder::JobConfigEncryptionSampleAes>>,
    /// Configuration for secrets stored in Google Secret Manager.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretManagerKeySource")]
    pub r#secret_manager_key_source: Option<Box<super::super::types::transcoder::JobConfigEncryptionSecretManagerKeySource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobConfigEncryption {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aes_128",
                    &self.r#aes_128,
                ),
                to_pulumi_object_field(
                    "drm_systems",
                    &self.r#drm_systems,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "mpeg_cenc",
                    &self.r#mpeg_cenc,
                ),
                to_pulumi_object_field(
                    "sample_aes",
                    &self.r#sample_aes,
                ),
                to_pulumi_object_field(
                    "secret_manager_key_source",
                    &self.r#secret_manager_key_source,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobConfigEncryption {
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
                    r#aes_128: {
                        let field_value = match fields_map.get("aes_128") {
                            Some(value) => value,
                            None => bail!("Missing field 'aes_128' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drm_systems: {
                        let field_value = match fields_map.get("drm_systems") {
                            Some(value) => value,
                            None => bail!("Missing field 'drm_systems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mpeg_cenc: {
                        let field_value = match fields_map.get("mpeg_cenc") {
                            Some(value) => value,
                            None => bail!("Missing field 'mpeg_cenc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_aes: {
                        let field_value = match fields_map.get("sample_aes") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_aes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_manager_key_source: {
                        let field_value = match fields_map.get("secret_manager_key_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_manager_key_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
