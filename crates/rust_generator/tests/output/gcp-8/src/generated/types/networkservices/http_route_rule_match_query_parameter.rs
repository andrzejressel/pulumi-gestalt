#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HttpRouteRuleMatchQueryParameter {
    /// The value of the query parameter must exactly match the contents of exactMatch.
    #[builder(into)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Option<String>,
    /// Specifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not.
    #[builder(into)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Option<bool>,
    /// The name of the query parameter to match.
    #[builder(into)]
    #[serde(rename = "queryParameter")]
    pub r#query_parameter: Option<String>,
    /// The value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Option<String>,
}
