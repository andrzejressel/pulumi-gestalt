#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrailAdvancedEventSelectorFieldSelector {
    /// A list of values that includes events that match the last few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "endsWiths")]
    pub r#ends_withs: Option<Vec<String>>,
    /// A list of values that includes events that match the exact value of the event record field specified as the value of `field`. This is the only valid operator that you can use with the `readOnly`, `eventCategory`, and `resources.type` fields.
    #[builder(into)]
    #[serde(rename = "equals")]
    pub r#equals: Option<Vec<String>>,
    /// Field in an event record on which to filter events to be logged. You can specify only the following values: `readOnly`, `eventSource`, `eventName`, `eventCategory`, `resources.type`, `resources.ARN`.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: String,
    /// A list of values that excludes events that match the last few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notEndsWiths")]
    pub r#not_ends_withs: Option<Vec<String>>,
    /// A list of values that excludes events that match the exact value of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notEquals")]
    pub r#not_equals: Option<Vec<String>>,
    /// A list of values that excludes events that match the first few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "notStartsWiths")]
    pub r#not_starts_withs: Option<Vec<String>>,
    /// A list of values that includes events that match the first few characters of the event record field specified as the value of `field`.
    #[builder(into)]
    #[serde(rename = "startsWiths")]
    pub r#starts_withs: Option<Vec<String>>,
}
