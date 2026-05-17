#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointKafkaSettings {
    /// Kafka broker location. Specify in the form broker-hostname-or-ip:port.
    #[builder(into)]
    #[serde(rename = "broker")]
    pub r#broker: String,
    /// Shows detailed control information for table definition, column definition, and table and column changes in the Kafka message output. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: Option<bool>,
    /// Include NULL and empty columns for records migrated to the endpoint. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: Option<bool>,
    /// Shows the partition value within the Kafka message output unless the partition type is `schema-table-type`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: Option<bool>,
    /// Includes any data definition language (DDL) operations that change the table in the control data, such as `rename-table`, `drop-table`, `add-column`, `drop-column`, and `rename-column`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: Option<bool>,
    /// Provides detailed transaction information from the source database. This information includes a commit timestamp, a log position, and values for `transaction_id`, previous `transaction_id`, and `transaction_record_id` (the record offset within a transaction). Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: Option<bool>,
    /// Output format for the records created on the endpoint. Message format is `JSON` (default) or `JSON_UNFORMATTED` (a single line with no tab).
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Option<String>,
    /// Maximum size in bytes for records created on the endpoint Default is `1,000,000`.
    #[builder(into)]
    #[serde(rename = "messageMaxBytes")]
    pub r#message_max_bytes: Option<i32>,
    /// Set this optional parameter to true to avoid adding a '0x' prefix to raw data in hexadecimal format. For example, by default, AWS DMS adds a '0x' prefix to the LOB column type in hexadecimal format moving from an Oracle source to a Kafka target. Use the `no_hex_prefix` endpoint setting to enable migration of RAW data type columns without adding the `'0x'` prefix.
    #[builder(into)]
    #[serde(rename = "noHexPrefix")]
    pub r#no_hex_prefix: Option<bool>,
    /// Prefixes schema and table names to partition values, when the partition type is `primary-key-type`. Doing this increases data distribution among Kafka partitions. For example, suppose that a SysBench schema has thousands of tables and each table has only limited range for a primary key. In this case, the same primary key is sent from thousands of tables to the same partition, which causes throttling. Default is `false`.
    #[builder(into)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: Option<bool>,
    /// Secure password you created when you first set up your MSK cluster to validate a client identity and make an encrypted connection between server and client using SASL-SSL authentication.
    #[builder(into)]
    #[serde(rename = "saslPassword")]
    pub r#sasl_password: Option<String>,
    /// Secure user name you created when you first set up your MSK cluster to validate a client identity and make an encrypted connection between server and client using SASL-SSL authentication.
    #[builder(into)]
    #[serde(rename = "saslUsername")]
    pub r#sasl_username: Option<String>,
    /// Set secure connection to a Kafka target endpoint using Transport Layer Security (TLS). Options include `ssl-encryption`, `ssl-authentication`, and `sasl-ssl`. `sasl-ssl` requires `sasl_username` and `sasl_password`.
    #[builder(into)]
    #[serde(rename = "securityProtocol")]
    pub r#security_protocol: Option<String>,
    /// ARN for the private certificate authority (CA) cert that AWS DMS uses to securely connect to your Kafka target endpoint.
    #[builder(into)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: Option<String>,
    /// ARN of the client certificate used to securely connect to a Kafka target endpoint.
    #[builder(into)]
    #[serde(rename = "sslClientCertificateArn")]
    pub r#ssl_client_certificate_arn: Option<String>,
    /// ARN for the client private key used to securely connect to a Kafka target endpoint.
    #[builder(into)]
    #[serde(rename = "sslClientKeyArn")]
    pub r#ssl_client_key_arn: Option<String>,
    /// Password for the client private key used to securely connect to a Kafka target endpoint.
    #[builder(into)]
    #[serde(rename = "sslClientKeyPassword")]
    pub r#ssl_client_key_password: Option<String>,
    /// Kafka topic for migration. Default is `kafka-default-topic`.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointKafkaSettings {
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
                    "broker",
                    &self.r#broker,
                ),
                to_pulumi_object_field(
                    "include_control_details",
                    &self.r#include_control_details,
                ),
                to_pulumi_object_field(
                    "include_null_and_empty",
                    &self.r#include_null_and_empty,
                ),
                to_pulumi_object_field(
                    "include_partition_value",
                    &self.r#include_partition_value,
                ),
                to_pulumi_object_field(
                    "include_table_alter_operations",
                    &self.r#include_table_alter_operations,
                ),
                to_pulumi_object_field(
                    "include_transaction_details",
                    &self.r#include_transaction_details,
                ),
                to_pulumi_object_field(
                    "message_format",
                    &self.r#message_format,
                ),
                to_pulumi_object_field(
                    "message_max_bytes",
                    &self.r#message_max_bytes,
                ),
                to_pulumi_object_field(
                    "no_hex_prefix",
                    &self.r#no_hex_prefix,
                ),
                to_pulumi_object_field(
                    "partition_include_schema_table",
                    &self.r#partition_include_schema_table,
                ),
                to_pulumi_object_field(
                    "sasl_password",
                    &self.r#sasl_password,
                ),
                to_pulumi_object_field(
                    "sasl_username",
                    &self.r#sasl_username,
                ),
                to_pulumi_object_field(
                    "security_protocol",
                    &self.r#security_protocol,
                ),
                to_pulumi_object_field(
                    "ssl_ca_certificate_arn",
                    &self.r#ssl_ca_certificate_arn,
                ),
                to_pulumi_object_field(
                    "ssl_client_certificate_arn",
                    &self.r#ssl_client_certificate_arn,
                ),
                to_pulumi_object_field(
                    "ssl_client_key_arn",
                    &self.r#ssl_client_key_arn,
                ),
                to_pulumi_object_field(
                    "ssl_client_key_password",
                    &self.r#ssl_client_key_password,
                ),
                to_pulumi_object_field(
                    "topic",
                    &self.r#topic,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointKafkaSettings {
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
                        let field_value = match fields_map.get("include_control_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_control_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_null_and_empty: {
                        let field_value = match fields_map.get("include_null_and_empty") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_null_and_empty' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_partition_value: {
                        let field_value = match fields_map.get("include_partition_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_partition_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_table_alter_operations: {
                        let field_value = match fields_map.get("include_table_alter_operations") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_table_alter_operations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_transaction_details: {
                        let field_value = match fields_map.get("include_transaction_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_transaction_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_format: {
                        let field_value = match fields_map.get("message_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_max_bytes: {
                        let field_value = match fields_map.get("message_max_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_max_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_hex_prefix: {
                        let field_value = match fields_map.get("no_hex_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_hex_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partition_include_schema_table: {
                        let field_value = match fields_map.get("partition_include_schema_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition_include_schema_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_password: {
                        let field_value = match fields_map.get("sasl_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'sasl_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sasl_username: {
                        let field_value = match fields_map.get("sasl_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'sasl_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_protocol: {
                        let field_value = match fields_map.get("security_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_ca_certificate_arn: {
                        let field_value = match fields_map.get("ssl_ca_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_ca_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_certificate_arn: {
                        let field_value = match fields_map.get("ssl_client_certificate_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_client_certificate_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_key_arn: {
                        let field_value = match fields_map.get("ssl_client_key_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_client_key_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_client_key_password: {
                        let field_value = match fields_map.get("ssl_client_key_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_client_key_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
