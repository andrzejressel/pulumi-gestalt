#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointKinesisSettings {
    /// Shows detailed control information for table definition, column definition, and table and column changes in the Kinesis message output. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: Option<bool>,
    /// Include NULL and empty columns in the target. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: Option<bool>,
    /// Shows the partition value within the Kinesis message output, unless the partition type is schema-table-type. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: Option<bool>,
    /// Includes any data definition language (DDL) operations that change the table in the control data. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: Option<bool>,
    /// Provides detailed transaction information from the source database. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: Option<bool>,
    /// Output format for the records created. Default is `json`. Valid values are `json` and `json-unformatted` (a single line with no tab).
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Option<String>,
    /// Prefixes schema and table names to partition values, when the partition type is primary-key-type. Default is `false`.
    #[builder(into)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: Option<bool>,
    /// ARN of the IAM Role with permissions to write to the Kinesis data stream.
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Option<String>,
    /// ARN of the Kinesis data stream.
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointKinesisSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
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
                    "partition_include_schema_table",
                    &self.r#partition_include_schema_table,
                ),
                to_pulumi_object_field(
                    "service_access_role_arn",
                    &self.r#service_access_role_arn,
                ),
                to_pulumi_object_field(
                    "stream_arn",
                    &self.r#stream_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointKinesisSettings {
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
                    r#partition_include_schema_table: {
                        let field_value = match fields_map.get("partition_include_schema_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition_include_schema_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_role_arn: {
                        let field_value = match fields_map.get("service_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_arn: {
                        let field_value = match fields_map.get("stream_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
