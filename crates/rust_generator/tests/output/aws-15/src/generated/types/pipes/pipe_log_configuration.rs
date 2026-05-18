#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeLogConfiguration {
    /// Amazon CloudWatch Logs logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsLogDestination")]
    pub r#cloudwatch_logs_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationCloudwatchLogsLogDestination>>,
    /// Amazon Kinesis Data Firehose logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "firehoseLogDestination")]
    pub r#firehose_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationFirehoseLogDestination>>,
    /// String list that specifies whether the execution data (specifically, the `payload`, `awsRequest`, and `awsResponse` fields) is included in the log messages for this pipe. This applies to all log destinations for the pipe. Valid values `ALL`.
    #[builder(into)]
    #[serde(rename = "includeExecutionDatas")]
    pub r#include_execution_datas: Option<Vec<String>>,
    /// The level of logging detail to include. Valid values `OFF`, `ERROR`, `INFO` and `TRACE`.
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: String,
    /// Amazon S3 logging configuration settings for the pipe. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3LogDestination")]
    pub r#s_3_log_destination: Option<Box<super::super::types::pipes::PipeLogConfigurationS3LogDestination>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeLogConfiguration {
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
                    "cloudwatch_logs_log_destination",
                    &self.r#cloudwatch_logs_log_destination,
                ),
                to_pulumi_object_field(
                    "firehose_log_destination",
                    &self.r#firehose_log_destination,
                ),
                to_pulumi_object_field(
                    "include_execution_datas",
                    &self.r#include_execution_datas,
                ),
                to_pulumi_object_field(
                    "level",
                    &self.r#level,
                ),
                to_pulumi_object_field(
                    "s_3_log_destination",
                    &self.r#s_3_log_destination,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeLogConfiguration {
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
                    r#cloudwatch_logs_log_destination: {
                        let field_value = match fields_map.get("cloudwatch_logs_log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logs_log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firehose_log_destination: {
                        let field_value = match fields_map.get("firehose_log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'firehose_log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_execution_datas: {
                        let field_value = match fields_map.get("include_execution_datas") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_execution_datas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level: {
                        let field_value = match fields_map.get("level") {
                            Some(value) => value,
                            None => bail!("Missing field 'level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_log_destination: {
                        let field_value = match fields_map.get("s_3_log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
