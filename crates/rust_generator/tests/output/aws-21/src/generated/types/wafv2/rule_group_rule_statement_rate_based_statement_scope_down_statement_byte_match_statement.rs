#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatement {
    /// The part of a web request that you want AWS WAF to inspect. See Field to Match below for details.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatementFieldToMatch>>,
    /// The area within the portion of a web request that you want AWS WAF to search for `search_string`. Valid values include the following: `EXACTLY`, `STARTS_WITH`, `ENDS_WITH`, `CONTAINS`, `CONTAINS_WORD`. See the AWS [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchStatement.html) for more information.
    #[builder(into)]
    #[serde(rename = "positionalConstraint")]
    pub r#positional_constraint: String,
    /// A string value that you want AWS WAF to search for. AWS WAF searches only in the part of web requests that you designate for inspection in `field_to_match`. The maximum length of the value is 50 bytes.
    #[builder(into)]
    #[serde(rename = "searchString")]
    pub r#search_string: String,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection.
    /// At least one required.
    /// See Text Transformation below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatementTextTransformation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("field_to_match".to_string(), self.r#field_to_match.to_pulumi_value().await);
            map.insert("positional_constraint".to_string(), self.r#positional_constraint.to_pulumi_value().await);
            map.insert("search_string".to_string(), self.r#search_string.to_pulumi_value().await);
            map.insert("text_transformations".to_string(), self.r#text_transformations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatement {
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
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatementFieldToMatch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#positional_constraint: {
                        let field_value = match fields_map.get("positional_constraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'positional_constraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#search_string: {
                        let field_value = match fields_map.get("search_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_transformations: {
                        let field_value = match fields_map.get("text_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementByteMatchStatementTextTransformation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
