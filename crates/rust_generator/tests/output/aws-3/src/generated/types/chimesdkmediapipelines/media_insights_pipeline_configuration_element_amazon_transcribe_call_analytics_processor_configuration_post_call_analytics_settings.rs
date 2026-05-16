#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings {
    /// Should output be redacted.
    #[builder(into)]
    #[serde(rename = "contentRedactionOutput")]
    pub r#content_redaction_output: Option<String>,
    /// ARN of the role used by AWS Transcribe to upload your post call analysis.
    #[builder(into)]
    #[serde(rename = "dataAccessRoleArn")]
    pub r#data_access_role_arn: String,
    /// ID of the KMS key used to encrypt the output.
    #[builder(into)]
    #[serde(rename = "outputEncryptionKmsKeyId")]
    pub r#output_encryption_kms_key_id: Option<String>,
    /// The Amazon S3 location where you want your Call Analytics post-call transcription output stored.
    #[builder(into)]
    #[serde(rename = "outputLocation")]
    pub r#output_location: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("content_redaction_output".to_string(), self.r#content_redaction_output.to_pulumi_value().await);
            map.insert("data_access_role_arn".to_string(), self.r#data_access_role_arn.to_pulumi_value().await);
            map.insert("output_encryption_kms_key_id".to_string(), self.r#output_encryption_kms_key_id.to_pulumi_value().await);
            map.insert("output_location".to_string(), self.r#output_location.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#content_redaction_output: {
                        let field_value = match fields_map.get("content_redaction_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_redaction_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#data_access_role_arn: {
                        let field_value = match fields_map.get("data_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_encryption_kms_key_id: {
                        let field_value = match fields_map.get("output_encryption_kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_encryption_kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_location: {
                        let field_value = match fields_map.get("output_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
