#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointDeliveryRuleDeviceCondition {
    /// Valid values are `Desktop` and `Mobile`.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Vec<String>,
    /// Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Option<bool>,
    /// Valid values are `Equal`. Defaults to `Equal`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
}
