#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRule {
    /// Action to perform in the ruleset rule. Available values: `block`, `challenge`, `compress_response`, `ddos_dynamic`, `ddos_mitigation`, `execute`, `force_connection_close`, `js_challenge`, `log`, `log_custom_field`, `managed_challenge`, `redirect`, `rewrite`, `route`, `score`, `serve_error`, `set_cache_settings`, `set_config`, `skip`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// List of parameters that configure the behavior of the ruleset rule action.
    #[builder(into)]
    #[serde(rename = "actionParameters")]
    pub r#action_parameters: Option<Box<super::types::RulesetRuleActionParameters>>,
    /// Brief summary of the ruleset rule and its intended use.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether the rule is active.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// List of parameters that configure exposed credential checks.
    #[builder(into)]
    #[serde(rename = "exposedCredentialCheck")]
    pub r#exposed_credential_check: Option<Box<super::types::RulesetRuleExposedCredentialCheck>>,
    /// Criteria for an HTTP request to trigger the ruleset rule action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: String,
    /// Unique rule identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The most recent update to this rule.
    #[builder(into)]
    #[serde(rename = "lastUpdated")]
    pub r#last_updated: Option<String>,
    /// List parameters to configure how the rule generates logs. Only valid for skip action.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::types::RulesetRuleLogging>>,
    /// List of parameters that configure HTTP rate limiting behaviour.
    #[builder(into)]
    #[serde(rename = "ratelimit")]
    pub r#ratelimit: Option<Box<super::types::RulesetRuleRatelimit>>,
    /// Rule reference.
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref_: Option<String>,
    /// Version of the ruleset to deploy.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRule {
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
                "action_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_parameters,
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "exposed_credential_check".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exposed_credential_check,
                )
                .await,
            );
            map.insert(
                "expression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expression,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "last_updated".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_updated,
                )
                .await,
            );
            map.insert(
                "logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#logging,
                )
                .await,
            );
            map.insert(
                "ratelimit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ratelimit,
                )
                .await,
            );
            map.insert(
                "ref_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ref_,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
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
                        let field_value = match fields_map.get("action_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("exposed_credential_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'exposed_credential_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("last_updated") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_updated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("ref_") {
                            Some(value) => value,
                            None => bail!("Missing field 'ref_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
