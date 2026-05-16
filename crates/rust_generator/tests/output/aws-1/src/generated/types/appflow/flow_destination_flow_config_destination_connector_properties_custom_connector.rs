#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector {
    #[builder(into)]
    #[serde(rename = "customProperties")]
    pub r#custom_properties: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "entityName")]
    pub r#entity_name: String,
    #[builder(into)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig>>,
    #[builder(into)]
    #[serde(rename = "idFieldNames")]
    pub r#id_field_names: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "writeOperationType")]
    pub r#write_operation_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("custom_properties".to_string(), self.r#custom_properties.to_pulumi_value().await);
            map.insert("entity_name".to_string(), self.r#entity_name.to_pulumi_value().await);
            map.insert("error_handling_config".to_string(), self.r#error_handling_config.to_pulumi_value().await);
            map.insert("id_field_names".to_string(), self.r#id_field_names.to_pulumi_value().await);
            map.insert("write_operation_type".to_string(), self.r#write_operation_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector {
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
                    r#custom_properties: {
                        let field_value = match fields_map.get("custom_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#entity_name: {
                        let field_value = match fields_map.get("entity_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#error_handling_config: {
                        let field_value = match fields_map.get("error_handling_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_handling_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnectorErrorHandlingConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id_field_names: {
                        let field_value = match fields_map.get("id_field_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'id_field_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#write_operation_type: {
                        let field_value = match fields_map.get("write_operation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_operation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
