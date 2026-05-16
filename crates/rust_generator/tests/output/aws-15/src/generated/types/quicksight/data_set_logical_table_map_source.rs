#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetLogicalTableMapSource {
    /// ARN of the parent data set.
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: Option<String>,
    /// Specifies the result of a join of two logical tables. See join_instruction.
    #[builder(into)]
    #[serde(rename = "joinInstruction")]
    pub r#join_instruction: Option<Box<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstruction>>,
    /// Physical table ID.
    #[builder(into)]
    #[serde(rename = "physicalTableId")]
    pub r#physical_table_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetLogicalTableMapSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_set_arn".to_string(), self.r#data_set_arn.to_pulumi_value().await);
            map.insert("join_instruction".to_string(), self.r#join_instruction.to_pulumi_value().await);
            map.insert("physical_table_id".to_string(), self.r#physical_table_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetLogicalTableMapSource {
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
                    r#data_set_arn: {
                        let field_value = match fields_map.get("data_set_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_set_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#join_instruction: {
                        let field_value = match fields_map.get("join_instruction") {
                            Some(value) => value,
                            None => bail!("Missing field 'join_instruction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DataSetLogicalTableMapSourceJoinInstruction>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#physical_table_id: {
                        let field_value = match fields_map.get("physical_table_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'physical_table_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
