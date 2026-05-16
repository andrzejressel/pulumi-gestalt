#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatement {
    /// Logical rule statement used to combine other rule statements with AND logic. See `and_statement` below for details.
    #[builder(into)]
    #[serde(rename = "andStatement")]
    pub r#and_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementAndStatement>>,
    /// Rule statement that defines a string match search for AWS WAF to apply to web requests. See `byte_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "byteMatchStatement")]
    pub r#byte_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementByteMatchStatement>>,
    /// Rule statement used to identify web requests based on country of origin. See `geo_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "geoMatchStatement")]
    pub r#geo_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementGeoMatchStatement>>,
    /// Rule statement used to detect web requests coming from particular IP addresses or address ranges. See `ip_set_reference_statement` below for details.
    #[builder(into)]
    #[serde(rename = "ipSetReferenceStatement")]
    pub r#ip_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementIpSetReferenceStatement>>,
    /// Rule statement that defines a string match search against labels that have been added to the web request by rules that have already run in the web ACL. See `label_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "labelMatchStatement")]
    pub r#label_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementLabelMatchStatement>>,
    /// Rule statement used to run the rules that are defined in a managed rule group.  This statement can not be nested. See `managed_rule_group_statement` below for details.
    #[builder(into)]
    #[serde(rename = "managedRuleGroupStatement")]
    pub r#managed_rule_group_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatement>>,
    /// Logical rule statement used to negate the results of another rule statement. See `not_statement` below for details.
    #[builder(into)]
    #[serde(rename = "notStatement")]
    pub r#not_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementNotStatement>>,
    /// Logical rule statement used to combine other rule statements with OR logic. See `or_statement` below for details.
    #[builder(into)]
    #[serde(rename = "orStatement")]
    pub r#or_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementOrStatement>>,
    /// Rate-based rule tracks the rate of requests for each originating `IP address`, and triggers the rule action when the rate exceeds a limit that you specify on the number of requests in any `5-minute` time span. This statement can not be nested. See `rate_based_statement` below for details.
    #[builder(into)]
    #[serde(rename = "rateBasedStatement")]
    pub r#rate_based_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatement>>,
    /// Rule statement used to search web request components for a match against a single regular expression. See `regex_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "regexMatchStatement")]
    pub r#regex_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexMatchStatement>>,
    /// Rule statement used to search web request components for matches with regular expressions. See `regex_pattern_set_reference_statement` below for details.
    #[builder(into)]
    #[serde(rename = "regexPatternSetReferenceStatement")]
    pub r#regex_pattern_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexPatternSetReferenceStatement>>,
    /// Rule statement used to run the rules that are defined in an WAFv2 Rule Group. See `rule_group_reference_statement` below for details.
    #[builder(into)]
    #[serde(rename = "ruleGroupReferenceStatement")]
    pub r#rule_group_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatement>>,
    /// Rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). See `size_constraint_statement` below for more details.
    #[builder(into)]
    #[serde(rename = "sizeConstraintStatement")]
    pub r#size_constraint_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatement>>,
    /// An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. See `sqli_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "sqliMatchStatement")]
    pub r#sqli_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatement>>,
    /// Rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. See `xss_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "xssMatchStatement")]
    pub r#xss_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementXssMatchStatement>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("and_statement".to_string(), self.r#and_statement.to_pulumi_value().await);
            map.insert("byte_match_statement".to_string(), self.r#byte_match_statement.to_pulumi_value().await);
            map.insert("geo_match_statement".to_string(), self.r#geo_match_statement.to_pulumi_value().await);
            map.insert("ip_set_reference_statement".to_string(), self.r#ip_set_reference_statement.to_pulumi_value().await);
            map.insert("label_match_statement".to_string(), self.r#label_match_statement.to_pulumi_value().await);
            map.insert("managed_rule_group_statement".to_string(), self.r#managed_rule_group_statement.to_pulumi_value().await);
            map.insert("not_statement".to_string(), self.r#not_statement.to_pulumi_value().await);
            map.insert("or_statement".to_string(), self.r#or_statement.to_pulumi_value().await);
            map.insert("rate_based_statement".to_string(), self.r#rate_based_statement.to_pulumi_value().await);
            map.insert("regex_match_statement".to_string(), self.r#regex_match_statement.to_pulumi_value().await);
            map.insert("regex_pattern_set_reference_statement".to_string(), self.r#regex_pattern_set_reference_statement.to_pulumi_value().await);
            map.insert("rule_group_reference_statement".to_string(), self.r#rule_group_reference_statement.to_pulumi_value().await);
            map.insert("size_constraint_statement".to_string(), self.r#size_constraint_statement.to_pulumi_value().await);
            map.insert("sqli_match_statement".to_string(), self.r#sqli_match_statement.to_pulumi_value().await);
            map.insert("xss_match_statement".to_string(), self.r#xss_match_statement.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatement {
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
                    r#and_statement: {
                        let field_value = match fields_map.get("and_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'and_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementAndStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#byte_match_statement: {
                        let field_value = match fields_map.get("byte_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'byte_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementByteMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#geo_match_statement: {
                        let field_value = match fields_map.get("geo_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'geo_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementGeoMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ip_set_reference_statement: {
                        let field_value = match fields_map.get("ip_set_reference_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_set_reference_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementIpSetReferenceStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#label_match_statement: {
                        let field_value = match fields_map.get("label_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementLabelMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#managed_rule_group_statement: {
                        let field_value = match fields_map.get("managed_rule_group_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_rule_group_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#not_statement: {
                        let field_value = match fields_map.get("not_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementNotStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#or_statement: {
                        let field_value = match fields_map.get("or_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'or_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementOrStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rate_based_statement: {
                        let field_value = match fields_map.get("rate_based_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'rate_based_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regex_match_statement: {
                        let field_value = match fields_map.get("regex_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regex_pattern_set_reference_statement: {
                        let field_value = match fields_map.get("regex_pattern_set_reference_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_pattern_set_reference_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexPatternSetReferenceStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rule_group_reference_statement: {
                        let field_value = match fields_map.get("rule_group_reference_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_group_reference_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#size_constraint_statement: {
                        let field_value = match fields_map.get("size_constraint_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_constraint_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sqli_match_statement: {
                        let field_value = match fields_map.get("sqli_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqli_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#xss_match_statement: {
                        let field_value = match fields_map.get("xss_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'xss_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementXssMatchStatement>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
