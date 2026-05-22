#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters {
    /// Configuration options for sending command output to CloudWatch Logs. Documented below.
    #[builder(into)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig>>,
    /// Information about the command(s) to execute.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.
    #[builder(into)]
    #[serde(rename = "documentHash")]
    pub r#document_hash: Option<String>,
    /// SHA-256 or SHA-1. SHA-1 hashes have been deprecated. Valid values: `Sha256` and `Sha1`
    #[builder(into)]
    #[serde(rename = "documentHashType")]
    pub r#document_hash_type: Option<String>,
    /// The version of an Automation document to use during task execution.
    #[builder(into)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Option<String>,
    /// Configurations for sending notifications about command status changes on a per-instance basis. Documented below.
    #[builder(into)]
    #[serde(rename = "notificationConfig")]
    pub r#notification_config: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig>>,
    /// The name of the Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "outputS3Bucket")]
    pub r#output_s_3_bucket: Option<String>,
    /// The Amazon S3 bucket subfolder.
    #[builder(into)]
    #[serde(rename = "outputS3KeyPrefix")]
    pub r#output_s_3_key_prefix: Option<String>,
    /// The parameters for the RUN_COMMAND task execution. Documented below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter>>,
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
    #[builder(into)]
    #[serde(rename = "serviceRoleArn")]
    pub r#service_role_arn: Option<String>,
    /// If this time is reached and the command has not already started executing, it doesn't run.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters {
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
                    "cloudwatchConfig",
                    &self.r#cloudwatch_config,
                ),
                to_pulumi_object_field(
                    "comment",
                    &self.r#comment,
                ),
                to_pulumi_object_field(
                    "documentHash",
                    &self.r#document_hash,
                ),
                to_pulumi_object_field(
                    "documentHashType",
                    &self.r#document_hash_type,
                ),
                to_pulumi_object_field(
                    "documentVersion",
                    &self.r#document_version,
                ),
                to_pulumi_object_field(
                    "notificationConfig",
                    &self.r#notification_config,
                ),
                to_pulumi_object_field(
                    "outputS3Bucket",
                    &self.r#output_s_3_bucket,
                ),
                to_pulumi_object_field(
                    "outputS3KeyPrefix",
                    &self.r#output_s_3_key_prefix,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "serviceRoleArn",
                    &self.r#service_role_arn,
                ),
                to_pulumi_object_field(
                    "timeoutSeconds",
                    &self.r#timeout_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters {
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
                    r#cloudwatch_config: {
                        let field_value = match fields_map.get("cloudwatchConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#comment: {
                        let field_value = match fields_map.get("comment") {
                            Some(value) => value,
                            None => bail!("Missing field 'comment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_hash: {
                        let field_value = match fields_map.get("documentHash") {
                            Some(value) => value,
                            None => bail!("Missing field 'documentHash' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_hash_type: {
                        let field_value = match fields_map.get("documentHashType") {
                            Some(value) => value,
                            None => bail!("Missing field 'documentHashType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_version: {
                        let field_value = match fields_map.get("documentVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'documentVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_config: {
                        let field_value = match fields_map.get("notificationConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'notificationConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_s_3_bucket: {
                        let field_value = match fields_map.get("outputS3Bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputS3Bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_s_3_key_prefix: {
                        let field_value = match fields_map.get("outputS3KeyPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputS3KeyPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_role_arn: {
                        let field_value = match fields_map.get("serviceRoleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceRoleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeoutSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
