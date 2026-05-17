#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyWithRulesRule {
    /// The Action to perform when the client connection triggers the rule. Can currently be either
    /// "allow", "deny", "apply_security_profile_group" or "goto_next".
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// A description of the rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The direction in which this rule applies. If unspecified an INGRESS rule is created.
    /// Possible values are: `INGRESS`, `EGRESS`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    /// Denotes whether the firewall policy rule is disabled. When set to true,
    /// the firewall policy rule is not enforced and traffic behaves as if it did
    /// not exist. If this is unspecified, the firewall policy rule will be
    /// enabled.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Denotes whether to enable logging for a particular rule.
    /// If logging is enabled, logs will be exported to the
    /// configured export destination in Stackdriver.
    #[builder(into)]
    #[serde(rename = "enableLogging")]
    pub r#enable_logging: Option<bool>,
    /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding 'action' is enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::compute::FirewallPolicyWithRulesRuleMatch>,
    /// An integer indicating the priority of a rule in the list. The priority must be a value
    /// between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the
    /// highest priority and 2147483647 is the lowest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// An optional name for the rule. This field is not a unique identifier
    /// and can be updated.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: Option<String>,
    /// A fully-qualified URL of a SecurityProfile resource instance.
    /// Example:
    /// https://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group
    /// Must be specified if action is 'apply_security_profile_group'.
    #[builder(into)]
    #[serde(rename = "securityProfileGroup")]
    pub r#security_profile_group: Option<String>,
    /// A list of network resource URLs to which this rule applies.
    /// This field allows you to control which network's VMs get
    /// this rule. If this field is left blank, all VMs
    /// within the organization will receive the rule.
    #[builder(into)]
    #[serde(rename = "targetResources")]
    pub r#target_resources: Option<Vec<String>>,
    /// A list of service accounts indicating the sets of
    /// instances that are applied with this rule.
    #[builder(into)]
    #[serde(rename = "targetServiceAccounts")]
    pub r#target_service_accounts: Option<Vec<String>>,
    /// Boolean flag indicating if the traffic should be TLS decrypted.
    /// It can be set only if action = 'apply_security_profile_group' and cannot be set for other actions.
    #[builder(into)]
    #[serde(rename = "tlsInspect")]
    pub r#tls_inspect: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyWithRulesRule {
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
                "direction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direction,
                )
                .await,
            );
            map.insert(
                "disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled,
                )
                .await,
            );
            map.insert(
                "enable_logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_logging,
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
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
                )
                .await,
            );
            map.insert(
                "rule_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_name,
                )
                .await,
            );
            map.insert(
                "security_profile_group".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_profile_group,
                )
                .await,
            );
            map.insert(
                "target_resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_resources,
                )
                .await,
            );
            map.insert(
                "target_service_accounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_service_accounts,
                )
                .await,
            );
            map.insert(
                "tls_inspect".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tls_inspect,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyWithRulesRule {
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
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_logging: {
                        let field_value = match fields_map.get("enable_logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_name: {
                        let field_value = match fields_map.get("rule_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_profile_group: {
                        let field_value = match fields_map.get("security_profile_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_profile_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resources: {
                        let field_value = match fields_map.get("target_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_service_accounts: {
                        let field_value = match fields_map.get("target_service_accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_service_accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_inspect: {
                        let field_value = match fields_map.get("tls_inspect") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_inspect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
