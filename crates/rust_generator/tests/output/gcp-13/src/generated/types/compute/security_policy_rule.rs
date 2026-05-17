#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyRule {
    /// Action to take when `match` matches the request. Valid values:
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// An optional description of this rule. Max size is 64.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Additional actions that are performed on headers. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Option<Box<super::super::types::compute::SecurityPolicyRuleHeaderAction>>,
    /// A match condition that incoming traffic is evaluated against.
    /// If it evaluates to true, the corresponding `action` is enforced. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::compute::SecurityPolicyRuleMatch>,
    /// Preconfigured WAF configuration to be applied for the rule. If the rule does not evaluate preconfigured WAF rules, i.e., if `evaluatePreconfiguredWaf()` is not used, this field will have no effect. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preconfiguredWafConfig")]
    pub r#preconfigured_waf_config: Option<Box<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfig>>,
    /// When set to true, the `action` specified above is not enforced.
    /// Stackdriver logs for requests that trigger a preview action are annotated as such.
    #[builder(into)]
    #[serde(rename = "preview")]
    pub r#preview: Option<bool>,
    /// An unique positive integer indicating the priority of evaluation for a rule.
    /// Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Must be specified if the `action` is `rate_based_ban` or `throttle`. Cannot be specified for other actions. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rateLimitOptions")]
    pub r#rate_limit_options: Option<Box<super::super::types::compute::SecurityPolicyRuleRateLimitOptions>>,
    /// Can be specified if the `action` is `redirect`. Cannot be specified for other actions. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redirectOptions")]
    pub r#redirect_options: Option<Box<super::super::types::compute::SecurityPolicyRuleRedirectOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityPolicyRule {
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
                    "action",
                    &self.r#action,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "header_action",
                    &self.r#header_action,
                ),
                to_pulumi_object_field(
                    "match_",
                    &self.r#match_,
                ),
                to_pulumi_object_field(
                    "preconfigured_waf_config",
                    &self.r#preconfigured_waf_config,
                ),
                to_pulumi_object_field(
                    "preview",
                    &self.r#preview,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "rate_limit_options",
                    &self.r#rate_limit_options,
                ),
                to_pulumi_object_field(
                    "redirect_options",
                    &self.r#redirect_options,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityPolicyRule {
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
                    r#header_action: {
                        let field_value = match fields_map.get("header_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_: {
                        let field_value = match fields_map.get("match_") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preconfigured_waf_config: {
                        let field_value = match fields_map.get("preconfigured_waf_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'preconfigured_waf_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
