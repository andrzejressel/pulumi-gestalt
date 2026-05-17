#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRateBasedStatement {
    /// Setting that indicates how to aggregate the request counts. Valid values include: `CONSTANT`, `CUSTOM_KEYS`, `FORWARDED_IP`, or `IP`. Default: `IP`.
    #[builder(into)]
    #[serde(rename = "aggregateKeyType")]
    pub r#aggregate_key_type: Option<String>,
    /// Aggregate the request counts using one or more web request components as the aggregate keys. See `custom_key` below for details.
    #[builder(into)]
    #[serde(rename = "customKeys")]
    pub r#custom_keys: Option<Vec<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementCustomKey>>,
    /// The amount of time, in seconds, that AWS WAF should include in its request counts, looking back from the current time. Valid values are `60`, `120`, `300`, and `600`. Defaults to `300` (5 minutes).
    /// 
    /// **NOTE:** This setting doesn't determine how often AWS WAF checks the rate, but how far back it looks each time it checks. AWS WAF checks the rate about every 10 seconds.
    #[builder(into)]
    #[serde(rename = "evaluationWindowSec")]
    pub r#evaluation_window_sec: Option<i32>,
    /// Configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. If `aggregate_key_type` is set to `FORWARDED_IP`, this block is required. See `forwarded_ip_config` below for details.
    #[builder(into)]
    #[serde(rename = "forwardedIpConfig")]
    pub r#forwarded_ip_config: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementForwardedIpConfig>>,
    /// Limit on requests per 5-minute period for a single originating IP address.
    #[builder(into)]
    #[serde(rename = "limit")]
    pub r#limit: i32,
    /// Optional nested statement that narrows the scope of the rate-based statement to matching web requests. This can be any nestable statement, and you can nest statements at any level below this scope-down statement. See `statement` above for details. If `aggregate_key_type` is set to `CONSTANT`, this block is required.
    #[builder(into)]
    #[serde(rename = "scopeDownStatement")]
    pub r#scope_down_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatement>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRateBasedStatement {
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
                "aggregate_key_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregate_key_type,
                )
                .await,
            );
            map.insert(
                "custom_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_keys,
                )
                .await,
            );
            map.insert(
                "evaluation_window_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_window_sec,
                )
                .await,
            );
            map.insert(
                "forwarded_ip_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwarded_ip_config,
                )
                .await,
            );
            map.insert(
                "limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#limit,
                )
                .await,
            );
            map.insert(
                "scope_down_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scope_down_statement,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRateBasedStatement {
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
                    r#aggregate_key_type: {
                        let field_value = match fields_map.get("aggregate_key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregate_key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_keys: {
                        let field_value = match fields_map.get("custom_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_window_sec: {
                        let field_value = match fields_map.get("evaluation_window_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_window_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarded_ip_config: {
                        let field_value = match fields_map.get("forwarded_ip_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_ip_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#limit: {
                        let field_value = match fields_map.get("limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope_down_statement: {
                        let field_value = match fields_map.get("scope_down_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope_down_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
