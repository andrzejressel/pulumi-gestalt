#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySetDefinitionPolicyDefinitionReference {
    /// Parameter values for the referenced policy rule. This field is a JSON string that allows you to assign parameters to this policy rule.
    #[builder(into)]
    #[serde(rename = "parameterValues")]
    pub r#parameter_values: Option<String>,
    /// The ID of the policy definition that will be included in this policy set definition.
    #[builder(into)]
    #[serde(rename = "policyDefinitionId")]
    pub r#policy_definition_id: String,
    /// A list of names of the policy definition groups that this policy definition reference belongs to.
    #[builder(into)]
    #[serde(rename = "policyGroupNames")]
    pub r#policy_group_names: Option<Vec<String>>,
    /// A unique ID within this policy set definition for this policy definition reference.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicySetDefinitionPolicyDefinitionReference {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "parameter_values",
                    &self.r#parameter_values,
                ),
                to_pulumi_object_field(
                    "policy_definition_id",
                    &self.r#policy_definition_id,
                ),
                to_pulumi_object_field(
                    "policy_group_names",
                    &self.r#policy_group_names,
                ),
                to_pulumi_object_field(
                    "reference_id",
                    &self.r#reference_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicySetDefinitionPolicyDefinitionReference {
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
                        let field_value = match fields_map.get("parameter_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameter_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_definition_id: {
                        let field_value = match fields_map.get("policy_definition_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_definition_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_group_names: {
                        let field_value = match fields_map.get("policy_group_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_group_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_id: {
                        let field_value = match fields_map.get("reference_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
