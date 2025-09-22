#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventSubscriptionAdvancedFilterStringEndsWith {
    /// Specifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Specifies an array of values to compare to when using a multiple values operator.
    /// 
    /// > **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
