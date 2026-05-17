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
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "object_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object_path,
                )
                .await,
            );
            map.insert(
                "pagination_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pagination_config,
                )
                .await,
            );
            map.insert(
                "parallelism_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parallelism_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowSourceFlowConfigSourceConnectorPropertiesSapoData {
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
                    r#object_path: {
                        let field_value = match fields_map.get("object_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pagination_config: {
                        let field_value = match fields_map.get("pagination_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'pagination_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parallelism_config: {
                        let field_value = match fields_map.get("parallelism_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
