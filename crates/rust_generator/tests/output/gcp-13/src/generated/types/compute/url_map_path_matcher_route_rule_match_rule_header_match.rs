#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch {
    /// The value should exactly match contents of exactMatch. Only one of exactMatch,
    /// prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[builder(into)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Option<String>,
    /// The name of the HTTP header to match. For matching against the HTTP request's
    /// authority, use a headerMatch with the header name ":authority". For matching a
    /// request's method, use the headerName ":method".
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// If set to false, the headerMatch is considered a match if the match criteria
    /// above are met. If set to true, the headerMatch is considered a match if the
    /// match criteria above are NOT met. Defaults to false.
    #[builder(into)]
    #[serde(rename = "invertMatch")]
    pub r#invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefixMatch. Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// A header with the contents of headerName must exist. The match takes place
    /// whether or not the request's header has a value or not. Only one of exactMatch,
    /// prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[builder(into)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Option<bool>,
    /// The header value must be an integer and its value must be in the range specified
    /// in rangeMatch. If the header does not contain an integer, number or is empty,
    /// the match fails. For example for a range [-5, 0]   - -3 will match.  - 0 will
    /// not match.  - 0.25 will not match.  - -3someString will not match.   Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rangeMatch")]
    pub r#range_match: Option<Box<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRuleHeaderMatchRangeMatch>>,
    /// The value of the header must match the regular expression specified in
    /// regexMatch. For regular expression grammar, please see:
    /// en.cppreference.com/w/cpp/regex/ecmascript  For matching against a port
    /// specified in the HTTP request, use a headerMatch with headerName set to PORT and
    /// a regular expression that satisfies the RFC2616 Host header's port specifier.
    /// Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or
    /// rangeMatch must be set.
    #[builder(into)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Option<String>,
    /// The value of the header must end with the contents of suffixMatch. Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    #[builder(into)]
    #[serde(rename = "suffixMatch")]
    pub r#suffix_match: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch {
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
                    "exact_match",
                    &self.r#exact_match,
                ),
                to_pulumi_object_field(
                    "header_name",
                    &self.r#header_name,
                ),
                to_pulumi_object_field(
                    "invert_match",
                    &self.r#invert_match,
                ),
                to_pulumi_object_field(
                    "prefix_match",
                    &self.r#prefix_match,
                ),
                to_pulumi_object_field(
                    "present_match",
                    &self.r#present_match,
                ),
                to_pulumi_object_field(
                    "range_match",
                    &self.r#range_match,
                ),
                to_pulumi_object_field(
                    "regex_match",
                    &self.r#regex_match,
                ),
                to_pulumi_object_field(
                    "suffix_match",
                    &self.r#suffix_match,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch {
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
                    r#exact_match: {
                        let field_value = match fields_map.get("exact_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'exact_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header_name: {
                        let field_value = match fields_map.get("header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invert_match: {
                        let field_value = match fields_map.get("invert_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'invert_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#present_match: {
                        let field_value = match fields_map.get("present_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'present_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_match: {
                        let field_value = match fields_map.get("range_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match: {
                        let field_value = match fields_map.get("regex_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suffix_match: {
                        let field_value = match fields_map.get("suffix_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'suffix_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
