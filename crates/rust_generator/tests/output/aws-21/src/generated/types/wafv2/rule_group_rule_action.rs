#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleAction {
    /// Instructs AWS WAF to allow the web request. See Allow below for details.
    #[builder(into)]
    #[serde(rename = "allow")]
    pub r#allow: Option<Box<super::super::types::wafv2::RuleGroupRuleActionAllow>>,
    /// Instructs AWS WAF to block the web request. See Block below for details.
    #[builder(into)]
    #[serde(rename = "block")]
    pub r#block: Option<Box<super::super::types::wafv2::RuleGroupRuleActionBlock>>,
    /// Instructs AWS WAF to run a `CAPTCHA` check against the web request. See Captcha below for details.
    #[builder(into)]
    #[serde(rename = "captcha")]
    pub r#captcha: Option<Box<super::super::types::wafv2::RuleGroupRuleActionCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See Challenge below for details.
    #[builder(into)]
    #[serde(rename = "challenge")]
    pub r#challenge: Option<Box<super::super::types::wafv2::RuleGroupRuleActionChallenge>>,
    /// Instructs AWS WAF to count the web request and allow it. See Count below for details.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<Box<super::super::types::wafv2::RuleGroupRuleActionCount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow".to_string(), self.r#allow.to_pulumi_value().await);
            map.insert("block".to_string(), self.r#block.to_pulumi_value().await);
            map.insert("captcha".to_string(), self.r#captcha.to_pulumi_value().await);
            map.insert("challenge".to_string(), self.r#challenge.to_pulumi_value().await);
            map.insert("count".to_string(), self.r#count.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleAction {
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
                    r#allow: {
                        let field_value = match fields_map.get("allow") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleActionAllow>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#block: {
                        let field_value = match fields_map.get("block") {
                            Some(value) => value,
                            None => bail!("Missing field 'block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleActionBlock>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#captcha: {
                        let field_value = match fields_map.get("captcha") {
                            Some(value) => value,
                            None => bail!("Missing field 'captcha' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleActionCaptcha>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#challenge: {
                        let field_value = match fields_map.get("challenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleActionChallenge>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleActionCount>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
