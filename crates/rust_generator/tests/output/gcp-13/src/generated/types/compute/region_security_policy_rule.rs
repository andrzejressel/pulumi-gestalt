#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRule {
    /// The Action to perform when the rule is matched. The following are the valid actions:
    /// * allow: allow access to target.
    /// * deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.
    /// * rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.
    /// * redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.
    /// * throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A match condition that incoming traffic is evaluated against.
    /// If it evaluates to true, the corresponding 'action' is enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleMatch>>,
    /// A match condition that incoming packets are evaluated against for CLOUD_ARMOR_NETWORK security policies. If it matches, the corresponding 'action' is enforced.
    /// The match criteria for a rule consists of built-in match fields (like 'srcIpRanges') and potentially multiple user-defined match fields ('userDefinedFields').
    /// Field values may be extracted directly from the packet or derived from it (e.g. 'srcRegionCodes'). Some fields may not be present in every packet (e.g. 'srcPorts'). A user-defined field is only present if the base header is found in the packet and the entire field is in bounds.
    /// Each match field may specify which values can match it, listing one or more ranges, prefixes, or exact values that are considered a match for the field. A field value must be present in order to match a specified match field. If no match values are specified for a match field, then any field value is considered to match it, and it's not required to be present. For strings specifying '*' is also equivalent to match all.
    /// For a packet to match a rule, all specified match fields must match the corresponding field values derived from the packet.
    /// Example:
    /// networkMatch: srcIpRanges: - "192.0.2.0/24" - "198.51.100.0/24" userDefinedFields: - name: "ipv4_fragment_offset" values: - "1-0x1fff"
    /// The above match condition matches packets with a source IP in 192.0.2.0/24 or 198.51.100.0/24 and a user-defined field named "ipv4_fragment_offset" with a value between 1 and 0x1fff inclusive
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkMatch")]
    pub r#network_match: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatch>>,
    /// Preconfigured WAF configuration to be applied for the rule.
    /// If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preconfiguredWafConfig")]
    pub r#preconfigured_waf_config: Option<Box<super::super::types::compute::RegionSecurityPolicyRulePreconfiguredWafConfig>>,
    /// If set to true, the specified action is not enforced.
    #[builder(into)]
    #[serde(rename = "preview")]
    pub r#preview: Option<bool>,
    /// An integer indicating the priority of a rule in the list.
    /// The priority must be a positive value between 0 and 2147483647.
    /// Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Must be specified if the action is "rate_based_ban" or "throttle". Cannot be specified for any other actions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rateLimitOptions")]
    pub r#rate_limit_options: Option<Box<super::super::types::compute::RegionSecurityPolicyRuleRateLimitOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionSecurityPolicyRule {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "match_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#match_,
                )
                .await,
            );
            map.insert(
                "network_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_match,
                )
                .await,
            );
            map.insert(
                "preconfigured_waf_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preconfigured_waf_config,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionSecurityPolicyRule {
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
                    r#match_: {
                        let field_value = match fields_map.get("match_") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_match: {
                        let field_value = match fields_map.get("network_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
