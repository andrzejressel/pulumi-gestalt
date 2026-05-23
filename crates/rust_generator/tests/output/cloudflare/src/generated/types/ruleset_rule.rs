#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[builder(into)]
    pub r#action: Option<String>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[builder(into)]
    pub r#action_parameters: Option<Box<super::types::RulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[builder(into)]
    pub r#description: Option<String>,
    /// Whether the rule is active.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// List of parameters that configure exposed credential checks.
    #[builder(into)]
    pub r#exposed_credential_check: Option<Box<super::types::RulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[builder(into)]
    pub r#expression: String,
    /// Unique rule identifier.
    #[builder(into)]
    pub r#id: Option<String>,
    /// The most recent update to this rule.
    #[builder(into)]
    pub r#last_updated: Option<String>,
    /// List parameters to configure how the rule generates logs. Only valid for skip action.
    #[builder(into)]
    pub r#logging: Option<Box<super::types::RulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[builder(into)]
    pub r#ratelimit: Option<Box<super::types::RulesetRuleRatelimit>>,
    /// Rule reference.
    #[builder(into)]
    pub r#ref_: Option<String>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRule {
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
                    "actionParameters",
                    &self.r#action_parameters,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exposedCredentialCheck",
                    &self.r#exposed_credential_check,
                ),
                to_pulumi_object_field(
                    "expression",
                    &self.r#expression,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "lastUpdated",
                    &self.r#last_updated,
                ),
                to_pulumi_object_field(
                    "logging",
                    &self.r#logging,
                ),
                to_pulumi_object_field(
                    "ratelimit",
                    &self.r#ratelimit,
                ),
                to_pulumi_object_field(
                    "ref",
                    &self.r#ref_,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RulesetRule {
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
                    r#action_parameters: {
                        let field_value = match fields_map.get("actionParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'actionParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exposed_credential_check: {
                        let field_value = match fields_map.get("exposedCredentialCheck") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposedCredentialCheck' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expression: {
                        let field_value = match fields_map.get("expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_updated: {
                        let field_value = match fields_map.get("lastUpdated") {
                            Some(value) => value,
                            None => bail!("Missing field 'lastUpdated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logging: {
                        let field_value = match fields_map.get("logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ratelimit: {
                        let field_value = match fields_map.get("ratelimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'ratelimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ref_: {
                        let field_value = match fields_map.get("ref") {
                            Some(value) => value,
                            None => bail!("Missing field 'ref' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
