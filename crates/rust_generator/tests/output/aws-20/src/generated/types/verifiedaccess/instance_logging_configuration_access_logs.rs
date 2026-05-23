#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceLoggingConfigurationAccessLogs {
    /// A block that specifies configures sending Verified Access logs to CloudWatch Logs. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsCloudwatchLogs>>,
    /// Include trust data sent by trust providers into the logs.
    #[builder(into)]
    #[serde(rename = "includeTrustContext")]
    pub r#include_trust_context: Option<bool>,
    /// A block that specifies configures sending Verified Access logs to Kinesis. Detailed below.
    #[builder(into)]
    #[serde(rename = "kinesisDataFirehose")]
    pub r#kinesis_data_firehose: Option<Box<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsKinesisDataFirehose>>,
    /// The logging version to use. Refer to [VerifiedAccessLogOptions](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_VerifiedAccessLogOptions.html) for the allowed values.
    #[builder(into)]
    #[serde(rename = "logVersion")]
    pub r#log_version: Option<String>,
    /// A block that specifies configures sending Verified Access logs to S3. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::verifiedaccess::InstanceLoggingConfigurationAccessLogsS3>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceLoggingConfigurationAccessLogs {
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
                    "cloudwatchLogs",
                    &self.r#cloudwatch_logs,
                ),
                to_pulumi_object_field(
                    "includeTrustContext",
                    &self.r#include_trust_context,
                ),
                to_pulumi_object_field(
                    "kinesisDataFirehose",
                    &self.r#kinesis_data_firehose,
                ),
                to_pulumi_object_field(
                    "logVersion",
                    &self.r#log_version,
                ),
                to_pulumi_object_field(
                    "s3",
                    &self.r#s_3,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceLoggingConfigurationAccessLogs {
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
                    r#cloudwatch_logs: {
                        let field_value = match fields_map.get("cloudwatchLogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchLogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_trust_context: {
                        let field_value = match fields_map.get("includeTrustContext") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeTrustContext' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_data_firehose: {
                        let field_value = match fields_map.get("kinesisDataFirehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisDataFirehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_version: {
                        let field_value = match fields_map.get("logVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'logVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s3") {
                            Some(value) => value,
                            None => bail!("Missing field 's3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
