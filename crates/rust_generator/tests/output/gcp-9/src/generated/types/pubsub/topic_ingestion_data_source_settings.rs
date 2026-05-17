#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicIngestionDataSourceSettings {
    /// Settings for ingestion from Amazon Kinesis Data Streams.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "awsKinesis")]
    pub r#aws_kinesis: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsAwsKinesis>>,
    /// Settings for ingestion from Cloud Storage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorage")]
    pub r#cloud_storage: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsCloudStorage>>,
    /// Settings for Platform Logs regarding ingestion to Pub/Sub. If unset,
    /// no Platform Logs will be generated.'
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "platformLogsSettings")]
    pub r#platform_logs_settings: Option<Box<super::super::types::pubsub::TopicIngestionDataSourceSettingsPlatformLogsSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicIngestionDataSourceSettings {
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
                    "aws_kinesis",
                    &self.r#aws_kinesis,
                ),
                to_pulumi_object_field(
                    "cloud_storage",
                    &self.r#cloud_storage,
                ),
                to_pulumi_object_field(
                    "platform_logs_settings",
                    &self.r#platform_logs_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicIngestionDataSourceSettings {
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
                    r#aws_kinesis: {
                        let field_value = match fields_map.get("aws_kinesis") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_kinesis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storage: {
                        let field_value = match fields_map.get("cloud_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
