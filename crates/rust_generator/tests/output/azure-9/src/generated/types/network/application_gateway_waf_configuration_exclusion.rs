#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayWafConfigurationExclusion {
    /// Match variable of the exclusion rule to exclude header, cookie or GET arguments. Possible values are `RequestArgKeys`, `RequestArgNames`, `RequestArgValues`, `RequestCookieKeys`, `RequestCookieNames`, `RequestCookieValues`, `RequestHeaderKeys`, `RequestHeaderNames` and `RequestHeaderValues`
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: String,
    /// String value which will be used for the filter operation. If empty will exclude all traffic on this `match_variable`
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<String>,
    /// Operator which will be used to search in the variable content. Possible values are `Contains`, `EndsWith`, `Equals`, `EqualsAny` and `StartsWith`. If empty will exclude all traffic on this `match_variable`
    #[builder(into)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Option<String>,
}
