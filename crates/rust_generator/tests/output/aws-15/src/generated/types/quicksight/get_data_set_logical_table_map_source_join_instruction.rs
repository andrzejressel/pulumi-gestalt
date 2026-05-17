#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetLogicalTableMapSourceJoinInstruction {
    #[builder(into)]
    #[serde(rename = "leftJoinKeyProperties")]
    pub r#left_join_key_properties: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionLeftJoinKeyProperty>,
    #[builder(into)]
    #[serde(rename = "leftOperand")]
    pub r#left_operand: String,
    #[builder(into)]
    #[serde(rename = "onClause")]
    pub r#on_clause: String,
    #[builder(into)]
    #[serde(rename = "rightJoinKeyProperties")]
    pub r#right_join_key_properties: Vec<super::super::types::quicksight::GetDataSetLogicalTableMapSourceJoinInstructionRightJoinKeyProperty>,
    #[builder(into)]
    #[serde(rename = "rightOperand")]
    pub r#right_operand: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataSetLogicalTableMapSourceJoinInstruction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "left_join_key_properties",
                    &self.r#left_join_key_properties,
                ),
                to_pulumi_object_field(
                    "left_operand",
                    &self.r#left_operand,
                ),
                to_pulumi_object_field(
                    "on_clause",
                    &self.r#on_clause,
                ),
                to_pulumi_object_field(
                    "right_join_key_properties",
                    &self.r#right_join_key_properties,
                ),
                to_pulumi_object_field(
                    "right_operand",
                    &self.r#right_operand,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataSetLogicalTableMapSourceJoinInstruction {
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
                    r#left_join_key_properties: {
                        let field_value = match fields_map.get("left_join_key_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'left_join_key_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#left_operand: {
                        let field_value = match fields_map.get("left_operand") {
                            Some(value) => value,
                            None => bail!("Missing field 'left_operand' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_clause: {
                        let field_value = match fields_map.get("on_clause") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_clause' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#right_join_key_properties: {
                        let field_value = match fields_map.get("right_join_key_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'right_join_key_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#right_operand: {
                        let field_value = match fields_map.get("right_operand") {
                            Some(value) => value,
                            None => bail!("Missing field 'right_operand' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
