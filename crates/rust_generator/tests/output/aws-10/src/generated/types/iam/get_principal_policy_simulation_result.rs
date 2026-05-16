#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrincipalPolicySimulationResult {
    /// The name of the single IAM action used for this particular request.
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: String,
    /// `true` if `decision` is "allowed", and `false` otherwise.
    #[builder(into)]
    #[serde(rename = "allowed")]
    pub r#allowed: bool,
    /// The raw decision determined from all of the policies in scope; either "allowed", "explicitDeny", or "implicitDeny".
    #[builder(into)]
    #[serde(rename = "decision")]
    pub r#decision: String,
    /// A map of arbitrary metadata entries returned by the policy simulator for this request.
    #[builder(into)]
    #[serde(rename = "decisionDetails")]
    pub r#decision_details: std::collections::HashMap<String, String>,
    /// A nested set of objects describing which policies contained statements that were relevant to this simulation request. Each object has attributes `source_policy_id` and `source_policy_type` to identify one of the policies.
    #[builder(into)]
    #[serde(rename = "matchedStatements")]
    pub r#matched_statements: Vec<super::super::types::iam::GetPrincipalPolicySimulationResultMatchedStatement>,
    /// A set of context keys (or condition keys) that were needed by some of the policies contributing to this result but not specified using a `context` block in the configuration. Missing or incorrect context keys will typically cause a simulated request to be disallowed.
    #[builder(into)]
    #[serde(rename = "missingContextKeys")]
    pub r#missing_context_keys: Vec<String>,
    /// ARN of the resource that was used for this particular request. When you specify multiple actions and multiple resource ARNs, that causes a separate policy request for each combination of unique action and resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPrincipalPolicySimulationResult {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action_name".to_string(), self.r#action_name.to_pulumi_value().await);
            map.insert("allowed".to_string(), self.r#allowed.to_pulumi_value().await);
            map.insert("decision".to_string(), self.r#decision.to_pulumi_value().await);
            map.insert("decision_details".to_string(), self.r#decision_details.to_pulumi_value().await);
            map.insert("matched_statements".to_string(), self.r#matched_statements.to_pulumi_value().await);
            map.insert("missing_context_keys".to_string(), self.r#missing_context_keys.to_pulumi_value().await);
            map.insert("resource_arn".to_string(), self.r#resource_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPrincipalPolicySimulationResult {
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
                    r#action_name: {
                        let field_value = match fields_map.get("action_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allowed: {
                        let field_value = match fields_map.get("allowed") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#decision: {
                        let field_value = match fields_map.get("decision") {
                            Some(value) => value,
                            None => bail!("Missing field 'decision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#decision_details: {
                        let field_value = match fields_map.get("decision_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'decision_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <std::collections::HashMap<String, String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#matched_statements: {
                        let field_value = match fields_map.get("matched_statements") {
                            Some(value) => value,
                            None => bail!("Missing field 'matched_statements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::iam::GetPrincipalPolicySimulationResultMatchedStatement> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#missing_context_keys: {
                        let field_value = match fields_map.get("missing_context_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'missing_context_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_arn: {
                        let field_value = match fields_map.get("resource_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
