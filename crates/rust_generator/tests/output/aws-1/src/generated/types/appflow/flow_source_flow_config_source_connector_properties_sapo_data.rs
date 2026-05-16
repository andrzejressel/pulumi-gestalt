#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSapoData {
    #[builder(into)]
    #[serde(rename = "objectPath")]
    pub r#object_path: String,
    /// Sets the page size for each concurrent process that transfers OData records from your SAP instance.
    #[builder(into)]
    #[serde(rename = "paginationConfig")]
    pub r#pagination_config: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig>>,
    /// Sets the number of concurrent processes that transfers OData records from your SAP instance.
    #[builder(into)]
    #[serde(rename = "parallelismConfig")]
    pub r#parallelism_config: Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowSourceFlowConfigSourceConnectorPropertiesSapoData {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("object_path".to_string(), self.r#object_path.to_pulumi_value().await);
            map.insert("pagination_config".to_string(), self.r#pagination_config.to_pulumi_value().await);
            map.insert("parallelism_config".to_string(), self.r#parallelism_config.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowSourceFlowConfigSourceConnectorPropertiesSapoData {
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
                    r#object_path: {
                        let field_value = match fields_map.get("object_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pagination_config: {
                        let field_value = match fields_map.get("pagination_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'pagination_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataPaginationConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parallelism_config: {
                        let field_value = match fields_map.get("parallelism_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
