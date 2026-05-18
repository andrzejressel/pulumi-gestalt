#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamRedshiftConfiguration {
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationCloudwatchLoggingOptions>>,
    /// The jdbcurl of the redshift cluster.
    #[builder(into)]
    #[serde(rename = "clusterJdbcurl")]
    pub r#cluster_jdbcurl: String,
    /// Copy options for copying the data from the s3 intermediate bucket into redshift, for example to change the default delimiter. For valid values, see the [AWS documentation](http://docs.aws.amazon.com/firehose/latest/APIReference/API_CopyCommand.html)
    #[builder(into)]
    #[serde(rename = "copyOptions")]
    pub r#copy_options: Option<String>,
    /// The data table columns that will be targeted by the copy command.
    #[builder(into)]
    #[serde(rename = "dataTableColumns")]
    pub r#data_table_columns: Option<String>,
    /// The name of the table in the redshift cluster that the s3 bucket will copy to.
    #[builder(into)]
    #[serde(rename = "dataTableName")]
    pub r#data_table_name: String,
    /// The password for the username above. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The data processing configuration.  See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationProcessingConfiguration>>,
    /// The length of time during which Firehose retries delivery after a failure, starting from the initial request and including the first attempt. The default value is 3600 seconds (60 minutes). Firehose does not retry if the value of DurationInSeconds is 0 (zero) or if the first delivery attempt takes longer than the current value.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// The arn of the role the stream assumes.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The configuration for backup in Amazon S3. Required if `s3_backup_mode` is `Enabled`. Supports the same fields as `s3_configuration` object.
    /// `secrets_manager_configuration` - (Optional) The Secrets Manager configuration. See `secrets_manager_configuration` block below for details. This value is required if `username` and `password` are not provided.
    #[builder(into)]
    #[serde(rename = "s3BackupConfiguration")]
    pub r#s_3_backup_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationS3BackupConfiguration>>,
    /// The Amazon S3 backup mode.  Valid values are `Disabled` and `Enabled`.  Default value is `Disabled`.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 Configuration. See s3_configuration below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationS3Configuration>,
    #[builder(into)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamRedshiftConfigurationSecretsManagerConfiguration>>,
    /// The username that the firehose delivery stream will assume. It is strongly recommended that the username and password provided is used exclusively for Amazon Kinesis Firehose purposes, and that the permissions for the account are restricted for Amazon Redshift INSERT permissions. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamRedshiftConfiguration {
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
                    "cloudwatch_logging_options",
                    &self.r#cloudwatch_logging_options,
                ),
                to_pulumi_object_field(
                    "cluster_jdbcurl",
                    &self.r#cluster_jdbcurl,
                ),
                to_pulumi_object_field(
                    "copy_options",
                    &self.r#copy_options,
                ),
                to_pulumi_object_field(
                    "data_table_columns",
                    &self.r#data_table_columns,
                ),
                to_pulumi_object_field(
                    "data_table_name",
                    &self.r#data_table_name,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "processing_configuration",
                    &self.r#processing_configuration,
                ),
                to_pulumi_object_field(
                    "retry_duration",
                    &self.r#retry_duration,
                ),
                to_pulumi_object_field(
                    "role_arn",
                    &self.r#role_arn,
                ),
                to_pulumi_object_field(
                    "s_3_backup_configuration",
                    &self.r#s_3_backup_configuration,
                ),
                to_pulumi_object_field(
                    "s_3_backup_mode",
                    &self.r#s_3_backup_mode,
                ),
                to_pulumi_object_field(
                    "s_3_configuration",
                    &self.r#s_3_configuration,
                ),
                to_pulumi_object_field(
                    "secrets_manager_configuration",
                    &self.r#secrets_manager_configuration,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamRedshiftConfiguration {
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
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatch_logging_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logging_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_jdbcurl: {
                        let field_value = match fields_map.get("cluster_jdbcurl") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_jdbcurl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copy_options: {
                        let field_value = match fields_map.get("copy_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_table_columns: {
                        let field_value = match fields_map.get("data_table_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_table_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_table_name: {
                        let field_value = match fields_map.get("data_table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retry_duration: {
                        let field_value = match fields_map.get("retry_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_backup_configuration: {
                        let field_value = match fields_map.get("s_3_backup_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_backup_mode: {
                        let field_value = match fields_map.get("s_3_backup_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_backup_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_configuration: {
                        let field_value = match fields_map.get("s_3_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_manager_configuration: {
                        let field_value = match fields_map.get("secrets_manager_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_manager_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
