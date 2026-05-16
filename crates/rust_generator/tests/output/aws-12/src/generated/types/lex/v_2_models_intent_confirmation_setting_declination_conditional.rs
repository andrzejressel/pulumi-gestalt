#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingDeclinationConditional {
    /// Whether a conditional branch is active. When active is false, the conditions are not evaluated.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// Configuration blocks for conditional branches. A conditional branch is made up of a condition, a response and a next step. The response and next step are executed when the condition is true. See `conditional_branch`.
    #[builder(into)]
    #[serde(rename = "conditionalBranches")]
    pub r#conditional_branches: Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalConditionalBranch>>,
    /// Configuration block for the conditional branch that should be followed when the conditions for other branches are not satisfied. A branch is made up of a condition, a response and a next step. See `default_branch`.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalDefaultBranch>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for V2ModelsIntentConfirmationSettingDeclinationConditional {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("active".to_string(), self.r#active.to_pulumi_value().await);
            map.insert("conditional_branches".to_string(), self.r#conditional_branches.to_pulumi_value().await);
            map.insert("default_branch".to_string(), self.r#default_branch.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for V2ModelsIntentConfirmationSettingDeclinationConditional {
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
                    r#active: {
                        let field_value = match fields_map.get("active") {
                            Some(value) => value,
                            None => bail!("Missing field 'active' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conditional_branches: {
                        let field_value = match fields_map.get("conditional_branches") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditional_branches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalConditionalBranch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_branch: {
                        let field_value = match fields_map.get("default_branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditionalDefaultBranch> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
