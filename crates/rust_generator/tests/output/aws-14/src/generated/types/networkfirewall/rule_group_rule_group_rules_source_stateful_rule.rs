#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRule {
    /// Action to take with packets in a traffic flow when the flow matches the stateful rule criteria. For all actions, AWS Network Firewall performs the specified action and discontinues stateful inspection of the traffic flow. Valid values: `ALERT`, `DROP`, `PASS`, or `REJECT`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// A configuration block containing the stateful 5-tuple inspection criteria for the rule, used to inspect traffic flows. See Header below for details.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleHeader>,
    /// Set of configuration blocks containing additional settings for a stateful rule. See Rule Option below for details.
    #[builder(into)]
    #[serde(rename = "ruleOptions")]
    pub r#rule_options: Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleRuleOption>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleGroupRulesSourceStatefulRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("action".to_string(), self.r#action.to_pulumi_value().await);
            map.insert("header".to_string(), self.r#header.to_pulumi_value().await);
            map.insert("rule_options".to_string(), self.r#rule_options.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleGroupRulesSourceStatefulRule {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#header: {
                        let field_value = match fields_map.get("header") {
                            Some(value) => value,
                            None => bail!("Missing field 'header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleHeader> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_options: {
                        let field_value = match fields_map.get("rule_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatefulRuleRuleOption> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
