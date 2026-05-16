#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionDynamodb {
    /// The hash key name.
    #[builder(into)]
    #[serde(rename = "hashKeyField")]
    pub r#hash_key_field: String,
    /// The hash key type. Valid values are "STRING" or "NUMBER".
    #[builder(into)]
    #[serde(rename = "hashKeyType")]
    pub r#hash_key_type: Option<String>,
    /// The hash key value.
    #[builder(into)]
    #[serde(rename = "hashKeyValue")]
    pub r#hash_key_value: String,
    /// The operation. Valid values are "INSERT", "UPDATE", or "DELETE".
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    /// The action payload.
    #[builder(into)]
    #[serde(rename = "payloadField")]
    pub r#payload_field: Option<String>,
    /// The range key name.
    #[builder(into)]
    #[serde(rename = "rangeKeyField")]
    pub r#range_key_field: Option<String>,
    /// The range key type. Valid values are "STRING" or "NUMBER".
    #[builder(into)]
    #[serde(rename = "rangeKeyType")]
    pub r#range_key_type: Option<String>,
    /// The range key value.
    #[builder(into)]
    #[serde(rename = "rangeKeyValue")]
    pub r#range_key_value: Option<String>,
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleErrorActionDynamodb {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("hash_key_field".to_string(), self.r#hash_key_field.to_pulumi_value().await);
            map.insert("hash_key_type".to_string(), self.r#hash_key_type.to_pulumi_value().await);
            map.insert("hash_key_value".to_string(), self.r#hash_key_value.to_pulumi_value().await);
            map.insert("operation".to_string(), self.r#operation.to_pulumi_value().await);
            map.insert("payload_field".to_string(), self.r#payload_field.to_pulumi_value().await);
            map.insert("range_key_field".to_string(), self.r#range_key_field.to_pulumi_value().await);
            map.insert("range_key_type".to_string(), self.r#range_key_type.to_pulumi_value().await);
            map.insert("range_key_value".to_string(), self.r#range_key_value.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);
            map.insert("table_name".to_string(), self.r#table_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleErrorActionDynamodb {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#hash_key_field: {
                        let field_value = match fields_map.get("hash_key_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_key_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hash_key_type: {
                        let field_value = match fields_map.get("hash_key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hash_key_value: {
                        let field_value = match fields_map.get("hash_key_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'hash_key_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#operation: {
                        let field_value = match fields_map.get("operation") {
                            Some(value) => value,
                            None => bail!("Missing field 'operation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#payload_field: {
                        let field_value = match fields_map.get("payload_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#range_key_field: {
                        let field_value = match fields_map.get("range_key_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_key_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#range_key_type: {
                        let field_value = match fields_map.get("range_key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#range_key_value: {
                        let field_value = match fields_map.get("range_key_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_key_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#table_name: {
                        let field_value = match fields_map.get("table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
