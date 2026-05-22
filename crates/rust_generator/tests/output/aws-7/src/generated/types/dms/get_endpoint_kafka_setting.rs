#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointKafkaSetting {
    #[builder(into)]
    #[serde(rename = "broker")]
    pub r#broker: String,
    #[builder(into)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: bool,
    #[builder(into)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: bool,
    #[builder(into)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: bool,
    #[builder(into)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: bool,
    #[builder(into)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: bool,
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: String,
    #[builder(into)]
    #[serde(rename = "messageMaxBytes")]
    pub r#message_max_bytes: i32,
    #[builder(into)]
    #[serde(rename = "noHexPrefix")]
    pub r#no_hex_prefix: bool,
    #[builder(into)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: bool,
    #[builder(into)]
    #[serde(rename = "saslPassword")]
    pub r#sasl_password: String,
    #[builder(into)]
    #[serde(rename = "saslUsername")]
    pub r#sasl_username: String,
    #[builder(into)]
    #[serde(rename = "securityProtocol")]
    pub r#security_protocol: String,
    #[builder(into)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientCertificateArn")]
    pub r#ssl_client_certificate_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientKeyArn")]
    pub r#ssl_client_key_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientKeyPassword")]
    pub r#ssl_client_key_password: String,
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointKafkaSetting {
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
                    "broker",
                    &self.r#broker,
                ),
                to_pulumi_object_field(
                    "includeControlDetails",
                    &self.r#include_control_details,
                ),
                to_pulumi_object_field(
                    "includeNullAndEmpty",
                    &self.r#include_null_and_empty,
                ),
                to_pulumi_object_field(
                    "includePartitionValue",
                    &self.r#include_partition_value,
                ),
                to_pulumi_object_field(
                    "includeTableAlterOperations",
                    &self.r#include_table_alter_operations,
                ),
                to_pulumi_object_field(
                    "includeTransactionDetails",
                    &self.r#include_transaction_details,
                ),
                to_pulumi_object_field(
                    "messageFormat",
                    &self.r#message_format,
                ),
                to_pulumi_object_field(
                    "messageMaxBytes",
                    &self.r#message_max_bytes,
                ),
                to_pulumi_object_field(
                    "noHexPrefix",
                    &self.r#no_hex_prefix,
                ),
                to_pulumi_object_field(
                    "partitionIncludeSchemaTable",
                    &self.r#partition_include_schema_table,
                ),
                to_pulumi_object_field(
                    "saslPassword",
                    &self.r#sasl_password,
                ),
                to_pulumi_object_field(
                    "saslUsername",
                    &self.r#sasl_username,
                ),
                to_pulumi_object_field(
                    "securityProtocol",
                    &self.r#security_protocol,
                ),
                to_pulumi_object_field(
                    "sslCaCertificateArn",
                    &self.r#ssl_ca_certificate_arn,
                ),
                to_pulumi_object_field(
                    "sslClientCertificateArn",
                    &self.r#ssl_client_certificate_arn,
                ),
                to_pulumi_object_field(
                    "sslClientKeyArn",
                    &self.r#ssl_client_key_arn,
                ),
                to_pulumi_object_field(
                    "sslClientKeyPassword",
                    &self.r#ssl_client_key_password,
                ),
                to_pulumi_object_field(
                    "topic",
                    &self.r#topic,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointKafkaSetting {
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
                    r#broker: {
                        let field_value = match fields_map.get("broker") {
                            Some(value) => value,
                            None => bail!("Missing field 'broker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_control_details: {
                        let field_value = match fields_map.get("includeControlDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeControlDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_null_and_empty: {
                        let field_value = match fields_map.get("includeNullAndEmpty") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeNullAndEmpty' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_partition_value: {
                        let field_value = match fields_map.get("includePartitionValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'includePartitionValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_table_alter_operations: {
                        let field_value = match fields_map.get("includeTableAlterOperations") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeTableAlterOperations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_transaction_details: {
                        let field_value = match fields_map.get("includeTransactionDetails") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeTransactionDetails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_format: {
                        let field_value = match fields_map.get("messageFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'messageFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_max_bytes: {
                        let field_value = match fields_map.get("messageMaxBytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'messageMaxBytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_hex_prefix: {
                        let field_value = match fields_map.get("noHexPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'noHexPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partition_include_schema_table: {
                        let field_value = match fields_map.get("partitionIncludeSchemaTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'partitionIncludeSchemaTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_password: {
                        let field_value = match fields_map.get("saslPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'saslPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_username: {
                        let field_value = match fields_map.get("saslUsername") {
                            Some(value) => value,
                            None => bail!("Missing field 'saslUsername' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_protocol: {
                        let field_value = match fields_map.get("securityProtocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityProtocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_ca_certificate_arn: {
                        let field_value = match fields_map.get("sslCaCertificateArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslCaCertificateArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_certificate_arn: {
                        let field_value = match fields_map.get("sslClientCertificateArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslClientCertificateArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_key_arn: {
                        let field_value = match fields_map.get("sslClientKeyArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslClientKeyArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_key_password: {
                        let field_value = match fields_map.get("sslClientKeyPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslClientKeyPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic: {
                        let field_value = match fields_map.get("topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
