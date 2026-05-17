#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamSnowflakeConfiguration {
    /// The URL of the Snowflake account. Format: https://[account_identifier].snowflakecomputing.com.
    #[builder(into)]
    #[serde(rename = "accountUrl")]
    pub r#account_url: String,
    /// Buffer incoming data for the specified period of time, in seconds between 0 to 900, before delivering it to the destination.  The default value is 0s.
    #[builder(into)]
    #[serde(rename = "bufferingInterval")]
    pub r#buffering_interval: Option<i32>,
    /// Buffer incoming data to the specified size, in MBs between 1 to 128, before delivering it to the destination.  The default value is 1MB.
    #[builder(into)]
    #[serde(rename = "bufferingSize")]
    pub r#buffering_size: Option<i32>,
    /// The CloudWatch Logging Options for the delivery stream. See `cloudwatch_logging_options` block below for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLoggingOptions")]
    pub r#cloudwatch_logging_options: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationCloudwatchLoggingOptions>>,
    /// The name of the content column.
    #[builder(into)]
    #[serde(rename = "contentColumnName")]
    pub r#content_column_name: Option<String>,
    /// The data loading option.
    #[builder(into)]
    #[serde(rename = "dataLoadingOption")]
    pub r#data_loading_option: Option<String>,
    /// The Snowflake database name.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: String,
    /// The passphrase for the private key.
    #[builder(into)]
    #[serde(rename = "keyPassphrase")]
    pub r#key_passphrase: Option<String>,
    /// The name of the metadata column.
    #[builder(into)]
    #[serde(rename = "metadataColumnName")]
    pub r#metadata_column_name: Option<String>,
    /// The private key for authentication. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    /// The processing configuration. See `processing_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationProcessingConfiguration>>,
    /// After an initial failure to deliver to Snowflake, the total amount of time, in seconds between 0 to 7200, during which Firehose re-attempts delivery (including the first attempt).  After this time has elapsed, the failed documents are written to Amazon S3.  The default value is 60s.  There will be no retry if the value is 0.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
    /// The ARN of the IAM role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The S3 backup mode.
    #[builder(into)]
    #[serde(rename = "s3BackupMode")]
    pub r#s_3_backup_mode: Option<String>,
    /// The S3 configuration. See `s3_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationS3Configuration>,
    /// The Snowflake schema name.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: String,
    /// The Secrets Manager configuration. See `secrets_manager_configuration` block below for details. This value is required if `user` and `private_key` are not provided.
    #[builder(into)]
    #[serde(rename = "secretsManagerConfiguration")]
    pub r#secrets_manager_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSecretsManagerConfiguration>>,
    /// The configuration for Snowflake role.
    #[builder(into)]
    #[serde(rename = "snowflakeRoleConfiguration")]
    pub r#snowflake_role_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeRoleConfiguration>>,
    /// The VPC configuration for Snowflake.
    #[builder(into)]
    #[serde(rename = "snowflakeVpcConfiguration")]
    pub r#snowflake_vpc_configuration: Option<Box<super::super::types::kinesis::FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeVpcConfiguration>>,
    /// The Snowflake table name.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: String,
    /// The user for authentication. This value is required if `secrets_manager_configuration` is not provided.
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamSnowflakeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "account_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_url,
                )
                .await,
            );
            map.insert(
                "buffering_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffering_interval,
                )
                .await,
            );
            map.insert(
                "buffering_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#buffering_size,
                )
                .await,
            );
            map.insert(
                "cloudwatch_logging_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_logging_options,
                )
                .await,
            );
            map.insert(
                "content_column_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_column_name,
                )
                .await,
            );
            map.insert(
                "data_loading_option".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_loading_option,
                )
                .await,
            );
            map.insert(
                "database".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database,
                )
                .await,
            );
            map.insert(
                "key_passphrase".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_passphrase,
                )
                .await,
            );
            map.insert(
                "metadata_column_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata_column_name,
                )
                .await,
            );
            map.insert(
                "private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_key,
                )
                .await,
            );
            map.insert(
                "processing_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#processing_configuration,
                )
                .await,
            );
            map.insert(
                "retry_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retry_duration,
                )
                .await,
            );
            map.insert(
                "role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_arn,
                )
                .await,
            );
            map.insert(
                "s_3_backup_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_backup_mode,
                )
                .await,
            );
            map.insert(
                "s_3_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_configuration,
                )
                .await,
            );
            map.insert(
                "schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema,
                )
                .await,
            );
            map.insert(
                "secrets_manager_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secrets_manager_configuration,
                )
                .await,
            );
            map.insert(
                "snowflake_role_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snowflake_role_configuration,
                )
                .await,
            );
            map.insert(
                "snowflake_vpc_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snowflake_vpc_configuration,
                )
                .await,
            );
            map.insert(
                "table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table,
                )
                .await,
            );
            map.insert(
                "user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamSnowflakeConfiguration {
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
                    r#account_url: {
                        let field_value = match fields_map.get("account_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffering_interval: {
                        let field_value = match fields_map.get("buffering_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#buffering_size: {
                        let field_value = match fields_map.get("buffering_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'buffering_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logging_options: {
                        let field_value = match fields_map.get("cloudwatch_logging_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logging_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_column_name: {
                        let field_value = match fields_map.get("content_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_loading_option: {
                        let field_value = match fields_map.get("data_loading_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_loading_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database: {
                        let field_value = match fields_map.get("database") {
                            Some(value) => value,
                            None => bail!("Missing field 'database' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_passphrase: {
                        let field_value = match fields_map.get("key_passphrase") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_passphrase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_column_name: {
                        let field_value = match fields_map.get("metadata_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_key: {
                        let field_value = match fields_map.get("private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#snowflake_role_configuration: {
                        let field_value = match fields_map.get("snowflake_role_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'snowflake_role_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snowflake_vpc_configuration: {
                        let field_value = match fields_map.get("snowflake_vpc_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'snowflake_vpc_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table: {
                        let field_value = match fields_map.get("table") {
                            Some(value) => value,
                            None => bail!("Missing field 'table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user: {
                        let field_value = match fields_map.get("user") {
                            Some(value) => value,
                            None => bail!("Missing field 'user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
