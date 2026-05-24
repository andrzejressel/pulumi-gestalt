#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicySetDefinitionPolicyDefinitionReference {
    /// The parameter values for the referenced policy rule. This field is a JSON object.
    #[builder(into)]
    pub r#parameter_values: String,
    /// The mapping of the parameter values for the referenced policy rule. The keys are the parameter names.
    #[builder(into)]
    pub r#parameters: std::collections::BTreeMap<String, String>,
    /// The ID of the policy definition or policy set definition that is included in this policy set definition.
    #[builder(into)]
    pub r#policy_definition_id: String,
    /// The list of names of the policy definition groups that this policy definition reference belongs to.
    #[builder(into)]
    pub r#policy_group_names: Vec<String>,
    /// The unique ID within this policy set definition for this policy definition reference.
    #[builder(into)]
    pub r#reference_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPolicySetDefinitionPolicyDefinitionReference {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "parameterValues",
                    &self.r#parameter_values,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "policyDefinitionId",
                    &self.r#policy_definition_id,
                ),
                to_pulumi_object_field(
                    "policyGroupNames",
                    &self.r#policy_group_names,
                ),
                to_pulumi_object_field(
                    "referenceId",
                    &self.r#reference_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPolicySetDefinitionPolicyDefinitionReference {
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
                    r#parameter_values: {
                        let field_value = match fields_map.get("parameterValues") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameterValues' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_definition_id: {
                        let field_value = match fields_map.get("policyDefinitionId") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyDefinitionId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_group_names: {
                        let field_value = match fields_map.get("policyGroupNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyGroupNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_id: {
                        let field_value = match fields_map.get("referenceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'referenceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
