#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatement {
    /// Logical rule statement used to combine other rule statements with AND logic. See `and_statement` below for details.
    #[builder(into)]
    #[serde(rename = "andStatement")]
    pub r#and_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementAndStatement>>,
    /// Rule statement that defines a string match search for AWS WAF to apply to web requests. See `byte_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "byteMatchStatement")]
    pub r#byte_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementByteMatchStatement>>,
    /// Rule statement used to identify web requests based on country of origin. See `geo_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "geoMatchStatement")]
    pub r#geo_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementGeoMatchStatement>>,
    /// Rule statement used to detect web requests coming from particular IP addresses or address ranges. See `ip_set_reference_statement` below for details.
    #[builder(into)]
    #[serde(rename = "ipSetReferenceStatement")]
    pub r#ip_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementIpSetReferenceStatement>>,
    /// Rule statement that defines a string match search against labels that have been added to the web request by rules that have already run in the web ACL. See `label_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "labelMatchStatement")]
    pub r#label_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementLabelMatchStatement>>,
    /// Logical rule statement used to negate the results of another rule statement. See `not_statement` below for details.
    #[builder(into)]
    #[serde(rename = "notStatement")]
    pub r#not_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementNotStatement>>,
    /// Logical rule statement used to combine other rule statements with OR logic. See `or_statement` below for details.
    #[builder(into)]
    #[serde(rename = "orStatement")]
    pub r#or_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementOrStatement>>,
    /// Rule statement used to search web request components for a match against a single regular expression. See `regex_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "regexMatchStatement")]
    pub r#regex_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexMatchStatement>>,
    /// Rule statement used to search web request components for matches with regular expressions. See `regex_pattern_set_reference_statement` below for details.
    #[builder(into)]
    #[serde(rename = "regexPatternSetReferenceStatement")]
    pub r#regex_pattern_set_reference_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatement>>,
    /// Rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). See `size_constraint_statement` below for more details.
    #[builder(into)]
    #[serde(rename = "sizeConstraintStatement")]
    pub r#size_constraint_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementSizeConstraintStatement>>,
    /// An SQL injection match condition identifies the part of web requests, such as the URI or the query string, that you want AWS WAF to inspect. See `sqli_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "sqliMatchStatement")]
    pub r#sqli_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementSqliMatchStatement>>,
    /// Rule statement that defines a cross-site scripting (XSS) match search for AWS WAF to apply to web requests. See `xss_match_statement` below for details.
    #[builder(into)]
    #[serde(rename = "xssMatchStatement")]
    pub r#xss_match_statement: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementXssMatchStatement>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementScopeDownStatement {
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
                "and_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#and_statement,
                )
                .await,
            );
            map.insert(
                "byte_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#byte_match_statement,
                )
                .await,
            );
            map.insert(
                "geo_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#geo_match_statement,
                )
                .await,
            );
            map.insert(
                "ip_set_reference_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_set_reference_statement,
                )
                .await,
            );
            map.insert(
                "label_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label_match_statement,
                )
                .await,
            );
            map.insert(
                "not_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_statement,
                )
                .await,
            );
            map.insert(
                "or_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#or_statement,
                )
                .await,
            );
            map.insert(
                "regex_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regex_match_statement,
                )
                .await,
            );
            map.insert(
                "regex_pattern_set_reference_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regex_pattern_set_reference_statement,
                )
                .await,
            );
            map.insert(
                "size_constraint_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#size_constraint_statement,
                )
                .await,
            );
            map.insert(
                "sqli_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sqli_match_statement,
                )
                .await,
            );
            map.insert(
                "xss_match_statement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#xss_match_statement,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementScopeDownStatement {
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
                        let field_value = match fields_map.get("and_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'and_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#byte_match_statement: {
                        let field_value = match fields_map.get("byte_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'byte_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geo_match_statement: {
                        let field_value = match fields_map.get("geo_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'geo_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_set_reference_statement: {
                        let field_value = match fields_map.get("ip_set_reference_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_set_reference_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_match_statement: {
                        let field_value = match fields_map.get("label_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_statement: {
                        let field_value = match fields_map.get("not_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#or_statement: {
                        let field_value = match fields_map.get("or_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'or_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match_statement: {
                        let field_value = match fields_map.get("regex_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_pattern_set_reference_statement: {
                        let field_value = match fields_map.get("regex_pattern_set_reference_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_pattern_set_reference_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_constraint_statement: {
                        let field_value = match fields_map.get("size_constraint_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_constraint_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqli_match_statement: {
                        let field_value = match fields_map.get("sqli_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqli_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xss_match_statement: {
                        let field_value = match fields_map.get("xss_match_statement") {
                            Some(value) => value,
                            None => bail!("Missing field 'xss_match_statement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
