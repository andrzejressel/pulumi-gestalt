#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatement {
    /// Operator to use to compare the request part to the size setting. Valid values include: `EQ`, `NE`, `LE`, `LT`, `GE`, or `GT`.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: String,
    /// Part of a web request that you want AWS WAF to inspect. See `field_to_match` below for details.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatementFieldToMatch>>,
    /// Size, in bytes, to compare to the request part, after any transformations. Valid values are integers between 0 and 21474836480, inclusive.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. At least one transformation is required. See `text_transformation` below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Vec<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatementTextTransformation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("comparison_operator".to_string(), self.r#comparison_operator.to_pulumi_value().await);
            map.insert("field_to_match".to_string(), self.r#field_to_match.to_pulumi_value().await);
            map.insert("size".to_string(), self.r#size.to_pulumi_value().await);
            map.insert("text_transformations".to_string(), self.r#text_transformations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatement {
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
                    r#comparison_operator: {
                        let field_value = match fields_map.get("comparison_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'comparison_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatementFieldToMatch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#size: {
                        let field_value = match fields_map.get("size") {
                            Some(value) => value,
                            None => bail!("Missing field 'size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_transformations: {
                        let field_value = match fields_map.get("text_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSizeConstraintStatementTextTransformation> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
