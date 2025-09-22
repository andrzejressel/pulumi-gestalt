#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTrafficPolicyDocumentRuleLocation {
    /// Value of a continent.
    #[builder(into)]
    #[serde(rename = "continent")]
    pub r#continent: Option<String>,
    /// Value of a country.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// References to an endpoint.
    #[builder(into)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Option<String>,
    /// Indicates whether you want Amazon Route 53 to evaluate the health of the endpoint and route traffic only to healthy endpoints.
    #[builder(into)]
    #[serde(rename = "evaluateTargetHealth")]
    pub r#evaluate_target_health: Option<bool>,
    /// If you want to associate a health check with the endpoint or rule.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<String>,
    /// Indicates whether this set of values represents the default location.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Option<bool>,
    /// References to a rule.
    #[builder(into)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Option<String>,
    /// Value of a subdivision.
    #[builder(into)]
    #[serde(rename = "subdivision")]
    pub r#subdivision: Option<String>,
}
