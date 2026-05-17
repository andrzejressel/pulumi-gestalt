#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityPolicyRulePreconfiguredWafConfigExclusion {
    /// Request cookie whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestCookies")]
    pub r#request_cookies: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestCooky>>,
    /// Request header whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestHeader>>,
    /// Request query parameter whose value will be excluded from inspection during preconfigured WAF evaluation.
    /// Note that the parameter can be in the query string or in the POST body.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestQueryParams")]
    pub r#request_query_params: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestQueryParam>>,
    /// Request URI from the request line to be excluded from inspection during preconfigured WAF evaluation.
    /// When specifying this field, the query or fragment part should be excluded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestUris")]
    pub r#request_uris: Option<Vec<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfigExclusionRequestUri>>,
    /// A list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.
    /// If omitted, it refers to all the rule IDs under the WAF rule set.
    #[builder(into)]
    #[serde(rename = "targetRuleIds")]
    pub r#target_rule_ids: Option<Vec<String>>,
    /// Target WAF rule set to apply the preconfigured WAF exclusion.
    #[builder(into)]
    #[serde(rename = "targetRuleSet")]
    pub r#target_rule_set: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityPolicyRulePreconfiguredWafConfigExclusion {
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
                    "request_cookies",
                    &self.r#request_cookies,
                ),
                to_pulumi_object_field(
                    "request_headers",
                    &self.r#request_headers,
                ),
                to_pulumi_object_field(
                    "request_query_params",
                    &self.r#request_query_params,
                ),
                to_pulumi_object_field(
                    "request_uris",
                    &self.r#request_uris,
                ),
                to_pulumi_object_field(
                    "target_rule_ids",
                    &self.r#target_rule_ids,
                ),
                to_pulumi_object_field(
                    "target_rule_set",
                    &self.r#target_rule_set,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityPolicyRulePreconfiguredWafConfigExclusion {
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
