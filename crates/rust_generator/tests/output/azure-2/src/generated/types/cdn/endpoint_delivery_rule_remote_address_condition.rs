#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRuleRemoteAddressCondition {
    /// List of string values. For `GeoMatch` `operator` this should be a list of country codes (e.g. `US` or `DE`). List of IP address if `operator` equals to `IPMatch`. This is required if `operator` is not `Any`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Option<Vec<String>>,
    /// Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Valid values are `Any`, `GeoMatch` and `IPMatch`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
}
