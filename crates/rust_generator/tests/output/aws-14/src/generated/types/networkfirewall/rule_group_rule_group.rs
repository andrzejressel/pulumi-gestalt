#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroup {
    /// A configuration block that defines the IP Set References for the rule group. See Reference Sets below for details. Please notes that there can only be a maximum of 5 `reference_sets` in a `rule_group`. See the [AWS documentation](https://docs.aws.amazon.com/network-firewall/latest/developerguide/rule-groups-ip-set-references.html#rule-groups-ip-set-reference-limits) for details.
    #[builder(into)]
    #[serde(rename = "referenceSets")]
    pub r#reference_sets: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupReferenceSets>>,
    /// A configuration block that defines additional settings available to use in the rules defined in the rule group. Can only be specified for **stateful** rule groups. See Rule Variables below for details.
    #[builder(into)]
    #[serde(rename = "ruleVariables")]
    pub r#rule_variables: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRuleVariables>>,
    /// A configuration block that defines the stateful or stateless rules for the rule group. See Rules Source below for details.
    #[builder(into)]
    #[serde(rename = "rulesSource")]
    pub r#rules_source: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSource>,
    /// A configuration block that defines stateful rule options for the rule group. See Stateful Rule Options below for details.
    #[builder(into)]
    #[serde(rename = "statefulRuleOptions")]
    pub r#stateful_rule_options: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupStatefulRuleOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleGroup {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "reference_sets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reference_sets,
                )
                .await,
            );
            map.insert(
                "rule_variables".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_variables,
                )
                .await,
            );
            map.insert(
                "rules_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rules_source,
                )
                .await,
            );
            map.insert(
                "stateful_rule_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stateful_rule_options,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleGroup {
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
                    r#reference_sets: {
                        let field_value = match fields_map.get("reference_sets") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_sets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_variables: {
                        let field_value = match fields_map.get("rule_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rules_source: {
                        let field_value = match fields_map.get("rules_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_rule_options: {
                        let field_value = match fields_map.get("stateful_rule_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_rule_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
