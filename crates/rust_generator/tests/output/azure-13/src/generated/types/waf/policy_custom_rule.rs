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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "group_rate_limit_by".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_rate_limit_by,
                )
                .await,
            );
            map.insert(
                "match_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#match_conditions,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
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
                "rate_limit_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_limit_duration,
                )
                .await,
            );
            map.insert(
                "rate_limit_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rate_limit_threshold,
                )
                .await,
            );
            map.insert(
                "rule_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_type,
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
