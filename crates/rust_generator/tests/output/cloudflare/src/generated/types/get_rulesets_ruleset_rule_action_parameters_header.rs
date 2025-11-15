#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions. Conflicts with `"value"`.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// Name of the HTTP request header to target.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    /// Static value to provide as the HTTP request header value. Conflicts with `"expression"`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
