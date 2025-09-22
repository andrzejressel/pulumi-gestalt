#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthzPolicyHttpRuleFromNotSourcePrincipal {
    /// The input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.
    /// Examples:
    /// * abc matches the value xyz.abc.def
    #[builder(into)]
    #[serde(rename = "contains")]
    pub r#contains: Option<String>,
    /// The input string must match exactly the string specified here.
    /// Examples:
    /// * abc only matches the value abc.
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Option<String>,
    /// If true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// The input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.
    /// Examples:
    /// * abc matches the value abc.xyz
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// The input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.
    /// Examples:
    /// * abc matches the value xyz.abc
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}
