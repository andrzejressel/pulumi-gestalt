#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoggingConfigurationLoggingConfigurationLogDestinationConfig {
    /// A map describing the logging destination for the chosen `log_destination_type`.
    /// * For an Amazon S3 bucket, specify the key `bucketName` with the name of the bucket and optionally specify the key `prefix` with a path (Do not add a leading / in the `prefix` as the configuration will have two // when applied).
    /// * For a CloudWatch log group, specify the key `logGroup` with the name of the CloudWatch log group.
    /// * For a Kinesis Data Firehose delivery stream, specify the key `deliveryStream` with the name of the delivery stream.
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: std::collections::HashMap<String, String>,
    /// The location to send logs to. Valid values: `S3`, `CloudWatchLogs`, `KinesisDataFirehose`.
    #[builder(into)]
    #[serde(rename = "logDestinationType")]
    pub r#log_destination_type: String,
    /// The type of log to send. Valid values: `ALERT` or `FLOW` or `TLS`. Alert logs report traffic that matches a `StatefulRule` with an action setting that sends a log message. Flow logs are standard network traffic flow logs.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoggingConfigurationLoggingConfigurationLogDestinationConfig {
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
                    "log_destination",
                    &self.r#log_destination,
                ),
                to_pulumi_object_field(
                    "log_destination_type",
                    &self.r#log_destination_type,
                ),
                to_pulumi_object_field(
                    "log_type",
                    &self.r#log_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoggingConfigurationLoggingConfigurationLogDestinationConfig {
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
                    r#log_destination: {
                        let field_value = match fields_map.get("log_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_destination_type: {
                        let field_value = match fields_map.get("log_destination_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_destination_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_type: {
                        let field_value = match fields_map.get("log_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
