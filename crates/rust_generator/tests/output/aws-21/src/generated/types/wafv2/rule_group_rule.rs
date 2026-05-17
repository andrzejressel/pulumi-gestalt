#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRule {
    /// The action that AWS WAF should take on a web request when it matches the rule's statement. Settings at the `aws.wafv2.WebAcl` level can override the rule action setting. See Action below for details.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::wafv2::RuleGroupRuleAction>,
    /// Specifies how AWS WAF should handle CAPTCHA evaluations. See Captcha Configuration below for details.
    #[builder(into)]
    #[serde(rename = "captchaConfig")]
    pub r#captcha_config: Option<Box<super::super::types::wafv2::RuleGroupRuleCaptchaConfig>>,
    /// A friendly name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// If you define more than one Rule in a WebACL, AWS WAF evaluates each request against the `rules` in order based on the value of `priority`. AWS WAF processes rules with lower priority first.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Labels to apply to web requests that match the rule match statement. See Rule Label below for details.
    #[builder(into)]
    #[serde(rename = "ruleLabels")]
    pub r#rule_labels: Option<Vec<super::super::types::wafv2::RuleGroupRuleRuleLabel>>,
    /// The AWS WAF processing statement for the rule, for example `byte_match_statement` or `geo_match_statement`. See Statement below for details.
    #[builder(into)]
    #[serde(rename = "statement")]
    pub r#statement: Box<super::super::types::wafv2::RuleGroupRuleStatement>,
    /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
    #[builder(into)]
    #[serde(rename = "visibilityConfig")]
    pub r#visibility_config: Box<super::super::types::wafv2::RuleGroupRuleVisibilityConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "action",
                    &self.r#action,
                ),
                to_pulumi_object_field(
                    "captcha_config",
                    &self.r#captcha_config,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "rule_labels",
                    &self.r#rule_labels,
                ),
                to_pulumi_object_field(
                    "statement",
                    &self.r#statement,
                ),
                to_pulumi_object_field(
                    "visibility_config",
                    &self.r#visibility_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRule {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#captcha_config: {
                        let field_value = match fields_map.get("captcha_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'captcha_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_labels: {
                        let field_value = match fields_map.get("rule_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statement: {
                        let field_value = match fields_map.get("statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#visibility_config: {
                        let field_value = match fields_map.get("visibility_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'visibility_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
