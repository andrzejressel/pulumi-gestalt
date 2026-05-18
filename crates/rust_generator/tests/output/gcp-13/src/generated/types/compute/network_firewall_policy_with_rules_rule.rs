#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkFirewallPolicyWithRulesRule {
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
    pub r#match_: Box<super::super::types::compute::NetworkFirewallPolicyWithRulesRuleMatch>,
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
    /// A list of secure tags that controls which instances the firewall rule
    /// applies to. If <code>targetSecureTag</code> are specified, then the
    /// firewall rule applies only to instances in the VPC network that have one
    /// of those EFFECTIVE secure tags, if all the target_secure_tag are in
    /// INEFFECTIVE state, then this rule will be ignored.
    /// <code>targetSecureTag</code> may not be set at the same time as
    /// <code>targetServiceAccounts</code>.
    /// If neither <code>targetServiceAccounts</code> nor
    /// <code>targetSecureTag</code> are specified, the firewall rule applies
    /// to all instances on the specified network.
    /// Maximum number of target label tags allowed is 256.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "targetSecureTags")]
    pub r#target_secure_tags: Option<Vec<super::super::types::compute::NetworkFirewallPolicyWithRulesRuleTargetSecureTag>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkFirewallPolicyWithRulesRule {
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
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "direction",
                    &self.r#direction,
                ),
                to_pulumi_object_field(
                    "disabled",
                    &self.r#disabled,
                ),
                to_pulumi_object_field(
                    "enable_logging",
                    &self.r#enable_logging,
                ),
                to_pulumi_object_field(
                    "match_",
                    &self.r#match_,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "rule_name",
                    &self.r#rule_name,
                ),
                to_pulumi_object_field(
                    "security_profile_group",
                    &self.r#security_profile_group,
                ),
                to_pulumi_object_field(
                    "target_secure_tags",
                    &self.r#target_secure_tags,
                ),
                to_pulumi_object_field(
                    "target_service_accounts",
                    &self.r#target_service_accounts,
                ),
                to_pulumi_object_field(
                    "tls_inspect",
                    &self.r#tls_inspect,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkFirewallPolicyWithRulesRule {
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
                    r#target_secure_tags: {
                        let field_value = match fields_map.get("target_secure_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_secure_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
