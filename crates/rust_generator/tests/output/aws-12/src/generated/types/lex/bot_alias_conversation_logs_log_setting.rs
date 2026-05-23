#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BotAliasConversationLogsLogSetting {
    /// The destination where logs are delivered. Options are `CLOUDWATCH_LOGS` or `S3`.
    #[builder(into)]
    pub r#destination: String,
    /// The Amazon Resource Name (ARN) of the key used to encrypt audio logs in an S3 bucket. This can only be specified when `destination` is set to `S3`. Must be between 20 and 2048 characters in length.
    #[builder(into)]
    pub r#kms_key_arn: Option<String>,
    /// The type of logging that is enabled. Options are `AUDIO` or `TEXT`.
    #[builder(into)]
    pub r#log_type: String,
    /// The Amazon Resource Name (ARN) of the CloudWatch Logs log group or S3 bucket where the logs are delivered. Must be less than or equal to 2048 characters in length.
    #[builder(into)]
    pub r#resource_arn: String,
    /// The prefix of the S3 object key for `AUDIO` logs or the log stream name for `TEXT` logs.
    #[builder(into)]
    pub r#resource_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BotAliasConversationLogsLogSetting {
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
                    "destination",
                    &self.r#destination,
                ),
                to_pulumi_object_field(
                    "kmsKeyArn",
                    &self.r#kms_key_arn,
                ),
                to_pulumi_object_field(
                    "logType",
                    &self.r#log_type,
                ),
                to_pulumi_object_field(
                    "resourceArn",
                    &self.r#resource_arn,
                ),
                to_pulumi_object_field(
                    "resourcePrefix",
                    &self.r#resource_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BotAliasConversationLogsLogSetting {
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
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_arn: {
                        let field_value = match fields_map.get("kmsKeyArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'kmsKeyArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_type: {
                        let field_value = match fields_map.get("logType") {
                            Some(value) => value,
                            None => bail!("Missing field 'logType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_arn: {
                        let field_value = match fields_map.get("resourceArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_prefix: {
                        let field_value = match fields_map.get("resourcePrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourcePrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
