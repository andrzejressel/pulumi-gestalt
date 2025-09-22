#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTrafficPolicyDocumentRuleGeoProximityLocation {
    /// Specify a value for `bias` if you want to route more traffic to an endpoint from nearby endpoints (positive values) or route less traffic to an endpoint (negative values).
    #[builder(into)]
    #[serde(rename = "bias")]
    pub r#bias: Option<String>,
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
    /// Represents the location south (negative) or north (positive) of the equator. Valid values are -90 degrees to 90 degrees.
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: Option<String>,
    /// Represents the location west (negative) or east (positive) of the prime meridian. Valid values are -180 degrees to 180 degrees.
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: Option<String>,
    /// If your endpoint is an AWS resource, specify the AWS Region that you created the resource in.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// References to a rule.
    #[builder(into)]
    #[serde(rename = "ruleReference")]
    pub r#rule_reference: Option<String>,
}
