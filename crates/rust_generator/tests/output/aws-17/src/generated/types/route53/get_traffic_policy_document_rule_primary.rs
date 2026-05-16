#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentRulePrimary {
    /// References to an endpoint.
    #[builder(into)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Option<String>,
    /// Indicates whether you want Amazon Route 53 to evaluate the health of the endpoint and route traffic only to healthy endpoints.
    #[builder(into)]
    #[serde(rename = "evaluateTargetHealth")]
    pub r#evaluate_target_health: Option<bool>,
    /// If you want to associate a health check with the endpoint or rule.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<String>,
    /// References to a rule.
    #[builder(into)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficPolicyDocumentRulePrimary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("endpoint_reference".to_string(), self.r#endpoint_reference.to_pulumi_value().await);
            map.insert("evaluate_target_health".to_string(), self.r#evaluate_target_health.to_pulumi_value().await);
            map.insert("health_check".to_string(), self.r#health_check.to_pulumi_value().await);
            map.insert("rule_reference".to_string(), self.r#rule_reference.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficPolicyDocumentRulePrimary {
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
                    r#endpoint_reference: {
                        let field_value = match fields_map.get("endpoint_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#evaluate_target_health: {
                        let field_value = match fields_map.get("evaluate_target_health") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluate_target_health' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#health_check: {
                        let field_value = match fields_map.get("health_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_reference: {
                        let field_value = match fields_map.get("rule_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
