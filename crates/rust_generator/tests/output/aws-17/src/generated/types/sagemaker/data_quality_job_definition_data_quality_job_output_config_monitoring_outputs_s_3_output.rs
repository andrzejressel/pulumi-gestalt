#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output {
    /// The local path to the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job. LocalPath is an absolute path for the output data. Defaults to `/opt/ml/processing/output`.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
    /// Whether to upload the results of the monitoring job continuously or after the job completes. Valid values are `Continuous` or `EndOfJob`
    #[builder(into)]
    #[serde(rename = "s3UploadMode")]
    pub r#s_3_upload_mode: Option<String>,
    /// A URI that identifies the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("local_path".to_string(), self.r#local_path.to_pulumi_value().await);
            map.insert("s_3_upload_mode".to_string(), self.r#s_3_upload_mode.to_pulumi_value().await);
            map.insert("s_3_uri".to_string(), self.r#s_3_uri.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output {
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
                    r#local_path: {
                        let field_value = match fields_map.get("local_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_upload_mode: {
                        let field_value = match fields_map.get("s_3_upload_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_upload_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_uri: {
                        let field_value = match fields_map.get("s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
