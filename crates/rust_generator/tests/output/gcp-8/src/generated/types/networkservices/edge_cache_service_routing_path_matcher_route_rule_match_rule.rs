#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
    /// For satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL.
    #[builder(into)]
    pub r#full_path_match: Option<String>,
    /// Specifies a list of header match criteria, all of which must match corresponding headers in the request.
    /// Structure is documented below.
    #[builder(into)]
    pub r#header_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleHeaderMatch>>,
    /// Specifies that prefixMatch and fullPathMatch matches are case sensitive.
    #[builder(into)]
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
    pub r#path_template_match: Option<String>,
    /// For satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into)]
    pub r#prefix_match: Option<String>,
    /// Specifies a list of query parameter match criteria, all of which must match corresponding query parameters in the request.
    /// Structure is documented below.
    #[builder(into)]
    pub r#query_parameter_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleQueryParameterMatch>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
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
                    "fullPathMatch",
                    &self.r#full_path_match,
                ),
                to_pulumi_object_field(
                    "headerMatches",
                    &self.r#header_matches,
                ),
                to_pulumi_object_field(
                    "ignoreCase",
                    &self.r#ignore_case,
                ),
                to_pulumi_object_field(
                    "pathTemplateMatch",
                    &self.r#path_template_match,
                ),
                to_pulumi_object_field(
                    "prefixMatch",
                    &self.r#prefix_match,
                ),
                to_pulumi_object_field(
                    "queryParameterMatches",
                    &self.r#query_parameter_matches,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("fullPathMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'fullPathMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_matches: {
                        let field_value = match fields_map.get("headerMatches") {
                            Some(value) => value,
                            None => bail!("Missing field 'headerMatches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_case: {
                        let field_value = match fields_map.get("ignoreCase") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreCase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_template_match: {
                        let field_value = match fields_map.get("pathTemplateMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'pathTemplateMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_match: {
                        let field_value = match fields_map.get("prefixMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefixMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_parameter_matches: {
                        let field_value = match fields_map.get("queryParameterMatches") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryParameterMatches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
