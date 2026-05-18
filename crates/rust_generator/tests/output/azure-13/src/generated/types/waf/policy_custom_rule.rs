#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyCustomRule {
    /// Type of action. Possible values are `Allow`, `Block` and `Log`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// Describes if the policy is in enabled state or disabled state. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Specifies what grouping the rate limit will count requests by. Possible values are `GeoLocation`, `ClientAddr` and `None`.
    #[builder(into)]
    #[serde(rename = "groupRateLimitBy")]
    pub r#group_rate_limit_by: Option<String>,
    /// One or more `match_conditions` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "matchConditions")]
    pub r#match_conditions: Vec<super::super::types::waf::PolicyCustomRuleMatchCondition>,
    /// Gets name of the resource that is unique within a policy. This name can be used to access the resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Describes priority of the rule. Rules with a lower value will be evaluated before rules with a higher value.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Specifies the duration at which the rate limit policy will be applied. Should be used with `RateLimitRule` rule type. Possible values are `FiveMins` and `OneMin`.
    #[builder(into)]
    #[serde(rename = "rateLimitDuration")]
    pub r#rate_limit_duration: Option<String>,
    /// Specifies the threshold value for the rate limit policy. Must be greater than or equal to 1 if provided.
    #[builder(into)]
    #[serde(rename = "rateLimitThreshold")]
    pub r#rate_limit_threshold: Option<i32>,
    /// Describes the type of rule. Possible values are `MatchRule`, `RateLimitRule` and `Invalid`.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyCustomRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "group_rate_limit_by",
                    &self.r#group_rate_limit_by,
                ),
                to_pulumi_object_field(
                    "match_conditions",
                    &self.r#match_conditions,
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
                    "rate_limit_duration",
                    &self.r#rate_limit_duration,
                ),
                to_pulumi_object_field(
                    "rate_limit_threshold",
                    &self.r#rate_limit_threshold,
                ),
                to_pulumi_object_field(
                    "rule_type",
                    &self.r#rule_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyCustomRule {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_rate_limit_by: {
                        let field_value = match fields_map.get("group_rate_limit_by") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_rate_limit_by' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_conditions: {
                        let field_value = match fields_map.get("match_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#rate_limit_duration: {
                        let field_value = match fields_map.get("rate_limit_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_limit_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_limit_threshold: {
                        let field_value = match fields_map.get("rate_limit_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_limit_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_type: {
                        let field_value = match fields_map.get("rule_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
