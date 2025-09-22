#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTrafficPolicyDocumentRule {
    /// Configuration block for when you add a geoproximity rule, you configure Amazon Route 53 to route traffic to your resources based on the geographic location of your resources. Only valid for `geoproximity` type. See below
    #[builder(into)]
    #[serde(rename = "geoProximityLocations")]
    pub r#geo_proximity_locations: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleGeoProximityLocation>>,
    /// ID of a rule you want to assign.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Configuration block for when you add a multivalue answer rule, you configure your traffic policy to route traffic approximately randomly to your healthy resources.  Only valid for `multivalue` type. See below
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleItem>>,
    /// Configuration block for when you add a geolocation rule, you configure your traffic policy to route your traffic based on the geographic location of your users.  Only valid for `geo` type. See below
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleLocation>>,
    /// Configuration block for the settings for the rule or endpoint that you want to route traffic to whenever the corresponding resources are available. Only valid for `failover` type. See below
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<Box<super::super::types::route53::GetTrafficPolicyDocumentRulePrimary>>,
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleRegion>>,
    /// Configuration block for the rule or endpoint that you want to route traffic to whenever the primary resources are not available. Only valid for `failover` type. See below
    #[builder(into)]
    #[serde(rename = "secondary")]
    pub r#secondary: Option<Box<super::super::types::route53::GetTrafficPolicyDocumentRuleSecondary>>,
    /// Type of the rule.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
