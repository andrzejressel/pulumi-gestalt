#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyRule {
    /// Action to take when match matches the request.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// An optional description of this rule. Max size is 64.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Additional actions that are performed on headers.
    #[builder(into)]
    #[serde(rename = "headerActions")]
    pub r#header_actions: Vec<super::super::types::compute::GetSecurityPolicyRuleHeaderAction>,
    /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding action is enforced.
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Vec<super::super::types::compute::GetSecurityPolicyRuleMatch>,
    /// Preconfigured WAF configuration to be applied for the rule. If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
    #[builder(into)]
    #[serde(rename = "preconfiguredWafConfigs")]
    pub r#preconfigured_waf_configs: Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfig>,
    /// When set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such.
    #[builder(into)]
    #[serde(rename = "preview")]
    pub r#preview: bool,
    /// An unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Rate limit threshold for this security policy. Must be specified if the action is "rate_based_ban" or "throttle". Cannot be specified for any other actions.
    #[builder(into)]
    #[serde(rename = "rateLimitOptions")]
    pub r#rate_limit_options: Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOption>,
    /// Parameters defining the redirect action. Cannot be specified for any other actions.
    #[builder(into)]
    #[serde(rename = "redirectOptions")]
    pub r#redirect_options: Vec<super::super::types::compute::GetSecurityPolicyRuleRedirectOption>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyRule {
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
                "action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "header_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_actions,
                )
                .await,
            );
            map.insert(
                "matches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#matches,
                )
                .await,
            );
            map.insert(
                "preconfigured_waf_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preconfigured_waf_configs,
                )
                .await,
            );
            map.insert(
                "preview".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preview,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
                )
                .await,
            );
            map.insert(
                "rate_limit_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_limit_options,
                )
                .await,
            );
            map.insert(
                "redirect_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_options,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyRule {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_actions: {
                        let field_value = match fields_map.get("header_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matches: {
                        let field_value = match fields_map.get("matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preconfigured_waf_configs: {
                        let field_value = match fields_map.get("preconfigured_waf_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'preconfigured_waf_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preview: {
                        let field_value = match fields_map.get("preview") {
                            Some(value) => value,
                            None => bail!("Missing field 'preview' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rate_limit_options: {
                        let field_value = match fields_map.get("rate_limit_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_limit_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_options: {
                        let field_value = match fields_map.get("redirect_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
