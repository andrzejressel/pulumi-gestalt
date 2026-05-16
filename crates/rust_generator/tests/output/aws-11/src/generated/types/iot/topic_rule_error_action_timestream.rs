#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionTimestream {
    /// The name of an Amazon Timestream database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// Configuration blocks with metadata attributes of the time series that are written in each measure record. Nested arguments below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Vec<super::super::types::iot::TopicRuleErrorActionTimestreamDimension>,
    /// The ARN of the role that grants permission to write to the Amazon Timestream database table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The name of the database table into which to write the measure records.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// Configuration block specifying an application-defined value to replace the default value assigned to the Timestream record's timestamp in the time column. Nested arguments below.
    #[builder(into)]
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<Box<super::super::types::iot::TopicRuleErrorActionTimestreamTimestamp>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleErrorActionTimestream {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("database_name".to_string(), self.r#database_name.to_pulumi_value().await);
            map.insert("dimensions".to_string(), self.r#dimensions.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);
            map.insert("table_name".to_string(), self.r#table_name.to_pulumi_value().await);
            map.insert("timestamp".to_string(), self.r#timestamp.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleErrorActionTimestream {
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
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::iot::TopicRuleErrorActionTimestreamDimension> as FromPulumiValue>::from_pulumi_value(field_value)?
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
                    r#timestamp: {
                        let field_value = match fields_map.get("timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionTimestreamTimestamp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
