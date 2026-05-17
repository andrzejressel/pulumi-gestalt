#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRegexMatchStatementFieldToMatchJsonBody {
    /// What to do when JSON parsing fails. Defaults to evaluating up to the first parsing failure. Valid values are `EVALUATE_AS_STRING`, `MATCH` and `NO_MATCH`.
    #[builder(into)]
    #[serde(rename = "invalidFallbackBehavior")]
    pub r#invalid_fallback_behavior: Option<String>,
    /// The patterns to look for in the JSON body. You must specify exactly one setting: either `all` or `included_paths`. See [JsonMatchPattern](https://docs.aws.amazon.com/waf/latest/APIReference/API_JsonMatchPattern.html) for details.
    #[builder(into)]
    #[serde(rename = "matchPattern")]
    pub r#match_pattern: Box<super::super::types::wafv2::WebAclRuleStatementRegexMatchStatementFieldToMatchJsonBodyMatchPattern>,
    /// The parts of the JSON to match against using the `match_pattern`. Valid values are `ALL`, `KEY` and `VALUE`.
    #[builder(into)]
    #[serde(rename = "matchScope")]
    pub r#match_scope: String,
    /// What to do if the body is larger than can be inspected. Valid values are `CONTINUE` (default), `MATCH` and `NO_MATCH`.
    #[builder(into)]
    #[serde(rename = "oversizeHandling")]
    pub r#oversize_handling: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRegexMatchStatementFieldToMatchJsonBody {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "invalid_fallback_behavior",
                    &self.r#invalid_fallback_behavior,
                ),
                to_pulumi_object_field(
                    "match_pattern",
                    &self.r#match_pattern,
                ),
                to_pulumi_object_field(
                    "match_scope",
                    &self.r#match_scope,
                ),
                to_pulumi_object_field(
                    "oversize_handling",
                    &self.r#oversize_handling,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRegexMatchStatementFieldToMatchJsonBody {
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
                    r#invalid_fallback_behavior: {
                        let field_value = match fields_map.get("invalid_fallback_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'invalid_fallback_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_pattern: {
                        let field_value = match fields_map.get("match_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_scope: {
                        let field_value = match fields_map.get("match_scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oversize_handling: {
                        let field_value = match fields_map.get("oversize_handling") {
                            Some(value) => value,
                            None => bail!("Missing field 'oversize_handling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
