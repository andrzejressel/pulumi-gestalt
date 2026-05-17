#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentRuleSecondary {
    #[builder(into)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Option<String>,
    #[builder(into)]
    #[serde(rename = "evaluateTargetHealth")]
    pub r#evaluate_target_health: Option<bool>,
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<String>,
    #[builder(into)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficPolicyDocumentRuleSecondary {
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
                "endpoint_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_reference,
                )
                .await,
            );
            map.insert(
                "evaluate_target_health".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluate_target_health,
                )
                .await,
            );
            map.insert(
                "health_check".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#health_check,
                )
                .await,
            );
            map.insert(
                "rule_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_reference,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficPolicyDocumentRuleSecondary {
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
                    r#endpoint_reference: {
                        let field_value = match fields_map.get("endpoint_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluate_target_health: {
                        let field_value = match fields_map.get("evaluate_target_health") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluate_target_health' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check: {
                        let field_value = match fields_map.get("health_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_reference: {
                        let field_value = match fields_map.get("rule_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
