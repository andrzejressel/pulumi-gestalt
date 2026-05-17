#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMapSource {
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: String,
    #[builder(into)]
    #[serde(rename = "joinInstructions")]
    pub r#join_instructions: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstruction>,
    #[builder(into)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetLogicalTableMapSource {
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
                "data_set_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_set_arn,
                )
                .await,
            );
            map.insert(
                "join_instructions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#join_instructions,
                )
                .await,
            );
            map.insert(
                "physical_table_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#physical_table_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetLogicalTableMapSource {
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
                    r#data_set_arn: {
                        let field_value = match fields_map.get("data_set_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_set_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#join_instructions: {
                        let field_value = match fields_map.get("join_instructions") {
                            Some(value) => value,
                            None => bail!("Missing field 'join_instructions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#physical_table_id: {
                        let field_value = match fields_map.get("physical_table_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'physical_table_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
