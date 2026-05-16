#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfig {
    /// Additional configuration for using the Account Creation Fraud Prevention managed rule group. Use this to specify information such as the registration page of your application and the type of content to accept or reject from the client.
    #[builder(into)]
    #[serde(rename = "awsManagedRulesAcfpRuleSet")]
    pub r#aws_managed_rules_acfp_rule_set: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet>>,
    /// Additional configuration for using the Account Takeover Protection managed rule group. Use this to specify information such as the sign-in page of your application and the type of content to accept or reject from the client.
    #[builder(into)]
    #[serde(rename = "awsManagedRulesAtpRuleSet")]
    pub r#aws_managed_rules_atp_rule_set: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet>>,
    /// Additional configuration for using the Bot Control managed rule group. Use this to specify the inspection level that you want to use. See `aws_managed_rules_bot_control_rule_set` for more details
    #[builder(into)]
    #[serde(rename = "awsManagedRulesBotControlRuleSet")]
    pub r#aws_managed_rules_bot_control_rule_set: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesBotControlRuleSet>>,
    /// The path of the login endpoint for your application.
    #[builder(into)]
    #[serde(rename = "loginPath")]
    pub r#login_path: Option<String>,
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigPasswordField>>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: Option<String>,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigUsernameField>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_managed_rules_acfp_rule_set".to_string(), self.r#aws_managed_rules_acfp_rule_set.to_pulumi_value().await);
            map.insert("aws_managed_rules_atp_rule_set".to_string(), self.r#aws_managed_rules_atp_rule_set.to_pulumi_value().await);
            map.insert("aws_managed_rules_bot_control_rule_set".to_string(), self.r#aws_managed_rules_bot_control_rule_set.to_pulumi_value().await);
            map.insert("login_path".to_string(), self.r#login_path.to_pulumi_value().await);
            map.insert("password_field".to_string(), self.r#password_field.to_pulumi_value().await);
            map.insert("payload_type".to_string(), self.r#payload_type.to_pulumi_value().await);
            map.insert("username_field".to_string(), self.r#username_field.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#aws_managed_rules_acfp_rule_set: {
                        let field_value = match fields_map.get("aws_managed_rules_acfp_rule_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_managed_rules_acfp_rule_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSet>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#aws_managed_rules_atp_rule_set: {
                        let field_value = match fields_map.get("aws_managed_rules_atp_rule_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_managed_rules_atp_rule_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSet>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#aws_managed_rules_bot_control_rule_set: {
                        let field_value = match fields_map.get("aws_managed_rules_bot_control_rule_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_managed_rules_bot_control_rule_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesBotControlRuleSet>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#login_path: {
                        let field_value = match fields_map.get("login_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#password_field: {
                        let field_value = match fields_map.get("password_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'password_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigPasswordField>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#payload_type: {
                        let field_value = match fields_map.get("payload_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#username_field: {
                        let field_value = match fields_map.get("username_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'username_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigUsernameField>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
