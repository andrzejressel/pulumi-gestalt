#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatement {
    /// Logical rule statement used to combine other rule statements with AND logic. See `and_statement` below for details.
    #[builder(into)]
    pub r#and_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementAndStatement>>,
    /// Rule statement that defines a string match search for AWS WAF to apply to web requests. See `byte_match_statement` below for details.
    #[builder(into)]
    pub r#byte_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementByteMatchStatement>>,
    /// Rule statement used to identify web requests based on country of origin. See `geo_match_statement` below for details.
    #[builder(into)]
    pub r#geo_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementGeoMatchStatement>>,
    /// Rule statement used to detect web requests coming from particular IP addresses or address ranges. See `ip_set_reference_statement` below for details.
    #[builder(into)]
    pub r#ip_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementIpSetReferenceStatement>>,
    /// Rule statement that defines a string match search against labels that have been added to the web request by rules that have already run in the web ACL. See `label_match_statement` below for details.
    #[builder(into)]
    pub r#label_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementLabelMatchStatement>>,
    /// Rule statement used to run the rules that are defined in a managed rule group.  This statement can not be nested. See `managed_rule_group_statement` below for details.
    #[builder(into)]
    pub r#managed_rule_group_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatement>>,
    /// Logical rule statement used to negate the results of another rule statement. See `not_statement` below for details.
    #[builder(into)]
    pub r#not_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementNotStatement>>,
    /// Logical rule statement used to combine other rule statements with OR logic. See `or_statement` below for details.
    #[builder(into)]
    pub r#or_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementOrStatement>>,
    /// Rate-based rule tracks the rate of requests for each originating `IP address`, and triggers the rule action when the rate exceeds a limit that you specify on the number of requests in any `5-minute` time span. This statement can not be nested. See `rate_based_statement` below for details.
    #[builder(into)]
    pub r#rate_based_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatement>>,
    /// Rule statement used to search web request components for a match against a single regular expression. See `regex_match_statement` below for details.
    #[builder(into)]
    pub r#regex_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexMatchStatement>>,
    /// Rule statement used to search web request components for matches with regular expressions. See `regex_pattern_set_reference_statement` below for details.
    #[builder(into)]
    pub r#regex_pattern_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRegexPatternSetReferenceStatement>>,
    /// Rule statement used to run the rules that are defined in an WAFv2 Rule Group. See `rule_group_reference_statement` below for details.
    #[builder(into)]
    pub r#rule_group_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatement>>,
    /// Rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). See `size_constraint_statement` below for more details.
    #[builder(into)]
    pub r#size_constraint_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatement>>,
    /// An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. See `sqli_match_statement` below for details.
    #[builder(into)]
    pub r#sqli_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementSqliMatchStatement>>,
    /// Rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. See `xss_match_statement` below for details.
    #[builder(into)]
    pub r#xss_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementXssMatchStatement>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatement {
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
                    "andStatement",
                    &self.r#and_statement,
                ),
                to_pulumi_object_field(
                    "byteMatchStatement",
                    &self.r#byte_match_statement,
                ),
                to_pulumi_object_field(
                    "geoMatchStatement",
                    &self.r#geo_match_statement,
                ),
                to_pulumi_object_field(
                    "ipSetReferenceStatement",
                    &self.r#ip_set_reference_statement,
                ),
                to_pulumi_object_field(
                    "labelMatchStatement",
                    &self.r#label_match_statement,
                ),
                to_pulumi_object_field(
                    "managedRuleGroupStatement",
                    &self.r#managed_rule_group_statement,
                ),
                to_pulumi_object_field(
                    "notStatement",
                    &self.r#not_statement,
                ),
                to_pulumi_object_field(
                    "orStatement",
                    &self.r#or_statement,
                ),
                to_pulumi_object_field(
                    "rateBasedStatement",
                    &self.r#rate_based_statement,
                ),
                to_pulumi_object_field(
                    "regexMatchStatement",
                    &self.r#regex_match_statement,
                ),
                to_pulumi_object_field(
                    "regexPatternSetReferenceStatement",
                    &self.r#regex_pattern_set_reference_statement,
                ),
                to_pulumi_object_field(
                    "ruleGroupReferenceStatement",
                    &self.r#rule_group_reference_statement,
                ),
                to_pulumi_object_field(
                    "sizeConstraintStatement",
                    &self.r#size_constraint_statement,
                ),
                to_pulumi_object_field(
                    "sqliMatchStatement",
                    &self.r#sqli_match_statement,
                ),
                to_pulumi_object_field(
                    "xssMatchStatement",
                    &self.r#xss_match_statement,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatement {
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
                    r#and_statement: {
                        let field_value = match fields_map.get("andStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'andStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#byte_match_statement: {
                        let field_value = match fields_map.get("byteMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'byteMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geo_match_statement: {
                        let field_value = match fields_map.get("geoMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'geoMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_set_reference_statement: {
                        let field_value = match fields_map.get("ipSetReferenceStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipSetReferenceStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_match_statement: {
                        let field_value = match fields_map.get("labelMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'labelMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_rule_group_statement: {
                        let field_value = match fields_map.get("managedRuleGroupStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedRuleGroupStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_statement: {
                        let field_value = match fields_map.get("notStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'notStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#or_statement: {
                        let field_value = match fields_map.get("orStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'orStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rate_based_statement: {
                        let field_value = match fields_map.get("rateBasedStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'rateBasedStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match_statement: {
                        let field_value = match fields_map.get("regexMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regexMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_pattern_set_reference_statement: {
                        let field_value = match fields_map.get("regexPatternSetReferenceStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regexPatternSetReferenceStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_group_reference_statement: {
                        let field_value = match fields_map.get("ruleGroupReferenceStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruleGroupReferenceStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_constraint_statement: {
                        let field_value = match fields_map.get("sizeConstraintStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'sizeConstraintStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqli_match_statement: {
                        let field_value = match fields_map.get("sqliMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqliMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xss_match_statement: {
                        let field_value = match fields_map.get("xssMatchStatement") {
                            Some(value) => value,
                            None => bail!("Missing field 'xssMatchStatement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
