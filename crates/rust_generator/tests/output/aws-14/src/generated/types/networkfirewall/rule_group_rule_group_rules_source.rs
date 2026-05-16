#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSource {
    /// A configuration block containing **stateful** inspection criteria for a domain list rule group. See Rules Source List below for details.
    #[builder(into)]
    #[serde(rename = "rulesSourceList")]
    pub r#rules_source_list: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceRulesSourceList>>,
    /// The fully qualified name of a file in an S3 bucket that contains Suricata compatible intrusion preventions system (IPS) rules or the Suricata rules as a string. These rules contain **stateful** inspection criteria and the action to take for traffic that matches the criteria.
    #[builder(into)]
    #[serde(rename = "rulesString")]
    pub r#rules_string: Option<String>,
    /// Set of configuration blocks containing **stateful** inspection criteria for 5-tuple rules to be used together in a rule group. See Stateful Rule below for details.
    #[builder(into)]
    #[serde(rename = "statefulRules")]
    pub r#stateful_rules: Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRule>>,
    /// A configuration block containing **stateless** inspection criteria for a stateless rule group. See Stateless Rules and Custom Actions below for details.
    #[builder(into)]
    #[serde(rename = "statelessRulesAndCustomActions")]
    pub r#stateless_rules_and_custom_actions: Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleGroupRulesSource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("rules_source_list".to_string(), self.r#rules_source_list.to_pulumi_value().await);
            map.insert("rules_string".to_string(), self.r#rules_string.to_pulumi_value().await);
            map.insert("stateful_rules".to_string(), self.r#stateful_rules.to_pulumi_value().await);
            map.insert("stateless_rules_and_custom_actions".to_string(), self.r#stateless_rules_and_custom_actions.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleGroupRulesSource {
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
                    r#rules_source_list: {
                        let field_value = match fields_map.get("rules_source_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules_source_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceRulesSourceList>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rules_string: {
                        let field_value = match fields_map.get("rules_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stateful_rules: {
                        let field_value = match fields_map.get("stateful_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateful_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRule>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stateless_rules_and_custom_actions: {
                        let field_value = match fields_map.get("stateless_rules_and_custom_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless_rules_and_custom_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
