#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTopicIngestionDataSourceSetting {
    /// Settings for ingestion from Amazon Kinesis Data Streams.
    #[builder(into)]
    #[serde(rename = "awsKineses")]
    pub r#aws_kineses: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingAwsKinese>,
    /// Settings for ingestion from Cloud Storage.
    #[builder(into)]
    #[serde(rename = "cloudStorages")]
    pub r#cloud_storages: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingCloudStorage>,
    /// Settings for Platform Logs regarding ingestion to Pub/Sub. If unset,
    /// no Platform Logs will be generated.'
    #[builder(into)]
    #[serde(rename = "platformLogsSettings")]
    pub r#platform_logs_settings: Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingPlatformLogsSetting>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTopicIngestionDataSourceSetting {
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
                "aws_kineses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aws_kineses,
                )
                .await,
            );
            map.insert(
                "cloud_storages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_storages,
                )
                .await,
            );
            map.insert(
                "platform_logs_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#platform_logs_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTopicIngestionDataSourceSetting {
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
                    r#aws_kineses: {
                        let field_value = match fields_map.get("aws_kineses") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_kineses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storages: {
                        let field_value = match fields_map.get("cloud_storages") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#platform_logs_settings: {
                        let field_value = match fields_map.get("platform_logs_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform_logs_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
