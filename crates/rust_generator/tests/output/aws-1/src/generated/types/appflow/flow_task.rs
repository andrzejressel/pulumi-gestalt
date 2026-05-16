#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowTask {
    /// Operation to be performed on the provided source fields. See Connector Operator for details.
    #[builder(into)]
    #[serde(rename = "connectorOperators")]
    pub r#connector_operators: Option<Vec<super::super::types::appflow::FlowTaskConnectorOperator>>,
    /// Field in a destination connector, or a field value against which Amazon AppFlow validates a source field.
    #[builder(into)]
    #[serde(rename = "destinationField")]
    pub r#destination_field: Option<String>,
    /// Source fields to which a particular task is applied.
    #[builder(into)]
    #[serde(rename = "sourceFields")]
    pub r#source_fields: Option<Vec<String>>,
    /// Map used to store task-related information. The execution service looks for particular information based on the `TaskType`. Valid keys are `VALUE`, `VALUES`, `DATA_TYPE`, `UPPER_BOUND`, `LOWER_BOUND`, `SOURCE_DATA_TYPE`, `DESTINATION_DATA_TYPE`, `VALIDATION_ACTION`, `MASK_VALUE`, `MASK_LENGTH`, `TRUNCATE_LENGTH`, `MATH_OPERATION_FIELDS_ORDER`, `CONCAT_FORMAT`, `SUBFIELD_CATEGORY_MAP`, and `EXCLUDE_SOURCE_FIELDS_LIST`.
    #[builder(into)]
    #[serde(rename = "taskProperties")]
    pub r#task_properties: Option<std::collections::HashMap<String, String>>,
    /// Particular task implementation that Amazon AppFlow performs. Valid values are `Arithmetic`, `Filter`, `Map`, `Map_all`, `Mask`, `Merge`, `Passthrough`, `Truncate`, and `Validate`.
    #[builder(into)]
    #[serde(rename = "taskType")]
    pub r#task_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowTask {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connector_operators".to_string(), self.r#connector_operators.to_pulumi_value().await);
            map.insert("destination_field".to_string(), self.r#destination_field.to_pulumi_value().await);
            map.insert("source_fields".to_string(), self.r#source_fields.to_pulumi_value().await);
            map.insert("task_properties".to_string(), self.r#task_properties.to_pulumi_value().await);
            map.insert("task_type".to_string(), self.r#task_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowTask {
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
                    r#connector_operators: {
                        let field_value = match fields_map.get("connector_operators") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_operators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::appflow::FlowTaskConnectorOperator>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination_field: {
                        let field_value = match fields_map.get("destination_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_fields: {
                        let field_value = match fields_map.get("source_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#task_properties: {
                        let field_value = match fields_map.get("task_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#task_type: {
                        let field_value = match fields_map.get("task_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
