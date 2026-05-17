#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataFlowSource {
    /// A `dataset` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Option<Box<super::super::types::datafactory::DataFlowSourceDataset>>,
    /// The description for the Data Flow Source.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A `flowlet` block as defined below.
    #[builder(into)]
    #[serde(rename = "flowlet")]
    pub r#flowlet: Option<Box<super::super::types::datafactory::DataFlowSourceFlowlet>>,
    /// A `linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "linkedService")]
    pub r#linked_service: Option<Box<super::super::types::datafactory::DataFlowSourceLinkedService>>,
    /// The name for the Data Flow Source.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `rejected_linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "rejectedLinkedService")]
    pub r#rejected_linked_service: Option<Box<super::super::types::datafactory::DataFlowSourceRejectedLinkedService>>,
    /// A `schema_linked_service` block as defined below.
    #[builder(into)]
    #[serde(rename = "schemaLinkedService")]
    pub r#schema_linked_service: Option<Box<super::super::types::datafactory::DataFlowSourceSchemaLinkedService>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataFlowSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "dataset".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dataset,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "flowlet".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flowlet,
                )
                .await,
            );
            map.insert(
                "linked_service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linked_service,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "rejected_linked_service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rejected_linked_service,
                )
                .await,
            );
            map.insert(
                "schema_linked_service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_linked_service,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataFlowSource {
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
                    r#dataset: {
                        let field_value = match fields_map.get("dataset") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flowlet: {
                        let field_value = match fields_map.get("flowlet") {
                            Some(value) => value,
                            None => bail!("Missing field 'flowlet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linked_service: {
                        let field_value = match fields_map.get("linked_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'linked_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rejected_linked_service: {
                        let field_value = match fields_map.get("rejected_linked_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'rejected_linked_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_linked_service: {
                        let field_value = match fields_map.get("schema_linked_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_linked_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
