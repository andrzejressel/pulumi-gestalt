#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecurityPolicyRulePreconfiguredWafConfigExclusion {
    /// Request cookie whose value will be excluded from inspection during preconfigured WAF evaluation.
    #[builder(into)]
    #[serde(rename = "requestCookies")]
    pub r#request_cookies: Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestCooky>,
    /// Request header whose value will be excluded from inspection during preconfigured WAF evaluation.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestHeader>,
    /// Request query parameter whose value will be excluded from inspection during preconfigured WAF evaluation.  Note that the parameter can be in the query string or in the POST body.
    #[builder(into)]
    #[serde(rename = "requestQueryParams")]
    pub r#request_query_params: Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam>,
    /// Request URI from the request line to be excluded from inspection during preconfigured WAF evaluation. When specifying this field, the query or fragment part should be excluded.
    #[builder(into)]
    #[serde(rename = "requestUris")]
    pub r#request_uris: Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri>,
    /// A list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion. If omitted, it refers to all the rule IDs under the WAF rule set.
    #[builder(into)]
    #[serde(rename = "targetRuleIds")]
    pub r#target_rule_ids: Vec<String>,
    /// Target WAF rule set to apply the preconfigured WAF exclusion.
    #[builder(into)]
    #[serde(rename = "targetRuleSet")]
    pub r#target_rule_set: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecurityPolicyRulePreconfiguredWafConfigExclusion {
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
                "request_cookies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_cookies,
                )
                .await,
            );
            map.insert(
                "request_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_headers,
                )
                .await,
            );
            map.insert(
                "request_query_params".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_query_params,
                )
                .await,
            );
            map.insert(
                "request_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_uris,
                )
                .await,
            );
            map.insert(
                "target_rule_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_rule_ids,
                )
                .await,
            );
            map.insert(
                "target_rule_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_rule_set,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecurityPolicyRulePreconfiguredWafConfigExclusion {
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
                    r#request_cookies: {
                        let field_value = match fields_map.get("request_cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_headers: {
                        let field_value = match fields_map.get("request_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_query_params: {
                        let field_value = match fields_map.get("request_query_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_query_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_uris: {
                        let field_value = match fields_map.get("request_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_rule_ids: {
                        let field_value = match fields_map.get("target_rule_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_rule_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_rule_set: {
                        let field_value = match fields_map.get("target_rule_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_rule_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
