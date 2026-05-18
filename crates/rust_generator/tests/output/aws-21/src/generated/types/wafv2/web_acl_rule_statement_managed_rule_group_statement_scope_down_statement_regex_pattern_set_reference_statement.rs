#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatement {
    /// The Amazon Resource Name (ARN) of the Regex Pattern Set that this statement references.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Part of a web request that you want AWS WAF to inspect. See `field_to_match` below for details.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatementFieldToMatch>>,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. At least one transformation is required. See `text_transformation` below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Vec<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatementTextTransformation>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatement {
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
                    "arn",
                    &self.r#arn,
                ),
                to_pulumi_object_field(
                    "field_to_match",
                    &self.r#field_to_match,
                ),
                to_pulumi_object_field(
                    "text_transformations",
                    &self.r#text_transformations,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatement {
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
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_transformations: {
                        let field_value = match fields_map.get("text_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
