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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "cloudwatch_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_config,
                )
                .await,
            );
            map.insert(
                "comment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#comment,
                )
                .await,
            );
            map.insert(
                "document_hash".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#document_hash,
                )
                .await,
            );
            map.insert(
                "document_hash_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#document_hash_type,
                )
                .await,
            );
            map.insert(
                "document_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#document_version,
                )
                .await,
            );
            map.insert(
                "notification_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_config,
                )
                .await,
            );
            map.insert(
                "output_s_3_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_s_3_bucket,
                )
                .await,
            );
            map.insert(
                "output_s_3_key_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_s_3_key_prefix,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "service_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_role_arn,
                )
                .await,
            );
            map.insert(
                "timeout_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_seconds,
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
                        let field_value = match fields_map.get("cloudwatch_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("document_hash") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_hash' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_hash_type: {
                        let field_value = match fields_map.get("document_hash_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_hash_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#document_version: {
                        let field_value = match fields_map.get("document_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'document_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_config: {
                        let field_value = match fields_map.get("notification_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_s_3_bucket: {
                        let field_value = match fields_map.get("output_s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_s_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_s_3_key_prefix: {
                        let field_value = match fields_map.get("output_s_3_key_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_s_3_key_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("service_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
