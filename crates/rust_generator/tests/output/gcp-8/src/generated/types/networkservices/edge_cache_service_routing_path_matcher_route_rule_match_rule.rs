#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
    /// For satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL.
    #[builder(into)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Option<String>,
    /// Specifies a list of header match criteria, all of which must match corresponding headers in the request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleHeaderMatch>>,
    /// Specifies that prefixMatch and fullPathMatch matches are case sensitive.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// For satisfying the matchRule condition, the path of the request
    /// must match the wildcard pattern specified in pathTemplateMatch
    /// after removing any query parameters and anchor that may be part
    /// of the original URL.
    /// pathTemplateMatch must be between 1 and 255 characters
    /// (inclusive).  The pattern specified by pathTemplateMatch may
    /// have at most 5 wildcard operators and at most 5 variable
    /// captures in total.
    #[builder(into)]
    #[serde(rename = "pathTemplateMatch")]
    pub r#path_template_match: Option<String>,
    /// For satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// Specifies a list of query parameter match criteria, all of which must match corresponding query parameters in the request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "queryParameterMatches")]
    pub r#query_parameter_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleQueryParameterMatch>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
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
                "full_path_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#full_path_match,
                )
                .await,
            );
            map.insert(
                "header_matches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header_matches,
                )
                .await,
            );
            map.insert(
                "ignore_case".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_case,
                )
                .await,
            );
            map.insert(
                "path_template_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_template_match,
                )
                .await,
            );
            map.insert(
                "prefix_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_match,
                )
                .await,
            );
            map.insert(
                "query_parameter_matches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_parameter_matches,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
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
                    r#full_path_match: {
                        let field_value = match fields_map.get("full_path_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_path_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_matches: {
                        let field_value = match fields_map.get("header_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_case: {
                        let field_value = match fields_map.get("ignore_case") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_case' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_template_match: {
                        let field_value = match fields_map.get("path_template_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_template_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_match: {
                        let field_value = match fields_map.get("prefix_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_parameter_matches: {
                        let field_value = match fields_map.get("query_parameter_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_parameter_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
