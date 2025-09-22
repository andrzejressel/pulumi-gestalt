#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementGeoMatchStatement {
    /// Array of two-character country codes, for example, [ "US", "CN" ], from the alpha-2 country ISO codes of the `ISO 3166` international standard. See the [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_GeoMatchStatement.html) for valid values.
    #[builder(into)]
    #[serde(rename = "countryCodes")]
    pub r#country_codes: Vec<String>,
    /// Configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. See `forwarded_ip_config` below for details.
    #[builder(into)]
    #[serde(rename = "forwardedIpConfig")]
    pub r#forwarded_ip_config: Option<Box<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementGeoMatchStatementForwardedIpConfig>>,
}
