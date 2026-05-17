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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "custom_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_properties,
                )
                .await,
            );
            map.insert(
                "entity_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#entity_name,
                )
                .await,
            );
            map.insert(
                "error_handling_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#error_handling_config,
                )
                .await,
            );
            map.insert(
                "id_field_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id_field_names,
                )
                .await,
            );
            map.insert(
                "write_operation_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_operation_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector {
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
                    r#custom_properties: {
                        let field_value = match fields_map.get("custom_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_name: {
                        let field_value = match fields_map.get("entity_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_handling_config: {
                        let field_value = match fields_map.get("error_handling_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_handling_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id_field_names: {
                        let field_value = match fields_map.get("id_field_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'id_field_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_operation_type: {
                        let field_value = match fields_map.get("write_operation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_operation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
