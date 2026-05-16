#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetActionDefinition {
    /// The AWS Identity and Access Management (IAM) action definition details. See IAM Action Definition.
    #[builder(into)]
    #[serde(rename = "iamActionDefinition")]
    pub r#iam_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionIamActionDefinition>>,
    /// The service control policies (SCPs) action definition details. See SCP Action Definition.
    #[builder(into)]
    #[serde(rename = "scpActionDefinition")]
    pub r#scp_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionScpActionDefinition>>,
    /// The AWS Systems Manager (SSM) action definition details. See SSM Action Definition.
    #[builder(into)]
    #[serde(rename = "ssmActionDefinition")]
    pub r#ssm_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionSsmActionDefinition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetActionDefinition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("iam_action_definition".to_string(), self.r#iam_action_definition.to_pulumi_value().await);
            map.insert("scp_action_definition".to_string(), self.r#scp_action_definition.to_pulumi_value().await);
            map.insert("ssm_action_definition".to_string(), self.r#ssm_action_definition.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetActionDefinition {
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
                    r#iam_action_definition: {
                        let field_value = match fields_map.get("iam_action_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_action_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::budgets::BudgetActionDefinitionIamActionDefinition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scp_action_definition: {
                        let field_value = match fields_map.get("scp_action_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'scp_action_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::budgets::BudgetActionDefinitionScpActionDefinition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ssm_action_definition: {
                        let field_value = match fields_map.get("ssm_action_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssm_action_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::budgets::BudgetActionDefinitionSsmActionDefinition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
