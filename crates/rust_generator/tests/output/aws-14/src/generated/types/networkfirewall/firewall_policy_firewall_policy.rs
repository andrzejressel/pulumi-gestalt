#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyFirewallPolicy {
    /// . Contains variables that you can use to override default Suricata settings in your firewall policy. See Rule Variables for details.
    #[builder(into)]
    #[serde(rename = "policyVariables")]
    pub r#policy_variables: Option<Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyPolicyVariables>>,
    /// Set of actions to take on a packet if it does not match any stateful rules in the policy. This can only be specified if the policy has a `stateful_engine_options` block with a `rule_order` value of `STRICT_ORDER`. You can specify one of either or neither values of `aws:drop_strict` or `aws:drop_established`, as well as any combination of `aws:alert_strict` and `aws:alert_established`.
    #[builder(into)]
    #[serde(rename = "statefulDefaultActions")]
    pub r#stateful_default_actions: Option<Vec<String>>,
    /// A configuration block that defines options on how the policy handles stateful rules. See Stateful Engine Options below for details.
    #[builder(into)]
    #[serde(rename = "statefulEngineOptions")]
    pub r#stateful_engine_options: Option<Box<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulEngineOptions>>,
    /// Set of configuration blocks containing references to the stateful rule groups that are used in the policy. See Stateful Rule Group Reference below for details.
    #[builder(into)]
    #[serde(rename = "statefulRuleGroupReferences")]
    pub r#stateful_rule_group_references: Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatefulRuleGroupReference>>,
    /// Set of configuration blocks describing the custom action definitions that are available for use in the firewall policy's `stateless_default_actions`. See Stateless Custom Action below for details.
    #[builder(into)]
    #[serde(rename = "statelessCustomActions")]
    pub r#stateless_custom_actions: Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatelessCustomAction>>,
    /// Set of actions to take on a packet if it does not match any of the stateless rules in the policy. You must specify one of the standard actions including: `aws:drop`, `aws:pass`, or `aws:forward_to_sfe`.
    /// In addition, you can specify custom actions that are compatible with your standard action choice. If you want non-matching packets to be forwarded for stateful inspection, specify `aws:forward_to_sfe`.
    #[builder(into)]
    #[serde(rename = "statelessDefaultActions")]
    pub r#stateless_default_actions: Vec<String>,
    /// Set of actions to take on a fragmented packet if it does not match any of the stateless rules in the policy. You must specify one of the standard actions including: `aws:drop`, `aws:pass`, or `aws:forward_to_sfe`.
    /// In addition, you can specify custom actions that are compatible with your standard action choice. If you want non-matching packets to be forwarded for stateful inspection, specify `aws:forward_to_sfe`.
    #[builder(into)]
    #[serde(rename = "statelessFragmentDefaultActions")]
    pub r#stateless_fragment_default_actions: Vec<String>,
    /// Set of configuration blocks containing references to the stateless rule groups that are used in the policy. See Stateless Rule Group Reference below for details.
    #[builder(into)]
    #[serde(rename = "statelessRuleGroupReferences")]
    pub r#stateless_rule_group_references: Option<Vec<super::super::types::networkfirewall::FirewallPolicyFirewallPolicyStatelessRuleGroupReference>>,
    /// The (ARN) of the TLS Inspection policy to attach to the FW Policy.  This must be added at creation of the resource per AWS documentation. "You can only add a TLS inspection configuration to a new policy, not to an existing policy."  This cannot be removed from a FW Policy.
    #[builder(into)]
    #[serde(rename = "tlsInspectionConfigurationArn")]
    pub r#tls_inspection_configuration_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyFirewallPolicy {
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
                    "policyVariables",
                    &self.r#policy_variables,
                ),
                to_pulumi_object_field(
                    "statefulDefaultActions",
                    &self.r#stateful_default_actions,
                ),
                to_pulumi_object_field(
                    "statefulEngineOptions",
                    &self.r#stateful_engine_options,
                ),
                to_pulumi_object_field(
                    "statefulRuleGroupReferences",
                    &self.r#stateful_rule_group_references,
                ),
                to_pulumi_object_field(
                    "statelessCustomActions",
                    &self.r#stateless_custom_actions,
                ),
                to_pulumi_object_field(
                    "statelessDefaultActions",
                    &self.r#stateless_default_actions,
                ),
                to_pulumi_object_field(
                    "statelessFragmentDefaultActions",
                    &self.r#stateless_fragment_default_actions,
                ),
                to_pulumi_object_field(
                    "statelessRuleGroupReferences",
                    &self.r#stateless_rule_group_references,
                ),
                to_pulumi_object_field(
                    "tlsInspectionConfigurationArn",
                    &self.r#tls_inspection_configuration_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyFirewallPolicy {
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
                    r#policy_variables: {
                        let field_value = match fields_map.get("policyVariables") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyVariables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_default_actions: {
                        let field_value = match fields_map.get("statefulDefaultActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'statefulDefaultActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_engine_options: {
                        let field_value = match fields_map.get("statefulEngineOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'statefulEngineOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateful_rule_group_references: {
                        let field_value = match fields_map.get("statefulRuleGroupReferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'statefulRuleGroupReferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_custom_actions: {
                        let field_value = match fields_map.get("statelessCustomActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'statelessCustomActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_default_actions: {
                        let field_value = match fields_map.get("statelessDefaultActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'statelessDefaultActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_fragment_default_actions: {
                        let field_value = match fields_map.get("statelessFragmentDefaultActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'statelessFragmentDefaultActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stateless_rule_group_references: {
                        let field_value = match fields_map.get("statelessRuleGroupReferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'statelessRuleGroupReferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_inspection_configuration_arn: {
                        let field_value = match fields_map.get("tlsInspectionConfigurationArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'tlsInspectionConfigurationArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
