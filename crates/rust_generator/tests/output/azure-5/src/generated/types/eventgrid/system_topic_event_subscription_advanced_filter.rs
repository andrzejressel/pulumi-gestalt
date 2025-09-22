#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SystemTopicEventSubscriptionAdvancedFilter {
    /// Compares a value of an event using a single boolean value.
    #[builder(into)]
    #[serde(rename = "boolEquals")]
    pub r#bool_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterBoolEqual>>,
    /// Evaluates if a value of an event isn't NULL or undefined.
    #[builder(into)]
    #[serde(rename = "isNotNulls")]
    pub r#is_not_nulls: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNotNull>>,
    /// Evaluates if a value of an event is NULL or undefined.
    /// 
    /// Each nested block consists of a key and a value(s) element.
    #[builder(into)]
    #[serde(rename = "isNullOrUndefineds")]
    pub r#is_null_or_undefineds: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefined>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberGreaterThanOrEquals")]
    pub r#number_greater_than_or_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberGreaterThans")]
    pub r#number_greater_thans: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    #[serde(rename = "numberInRanges")]
    pub r#number_in_ranges: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    #[serde(rename = "numberIns")]
    pub r#number_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberIn>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberLessThanOrEquals")]
    pub r#number_less_than_or_equals: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual>>,
    /// Compares a value of an event using a single floating point number.
    #[builder(into)]
    #[serde(rename = "numberLessThans")]
    pub r#number_less_thans: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberLessThan>>,
    /// Compares a value of an event using multiple floating point number ranges.
    #[builder(into)]
    #[serde(rename = "numberNotInRanges")]
    pub r#number_not_in_ranges: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotInRange>>,
    /// Compares a value of an event using multiple floating point numbers.
    #[builder(into)]
    #[serde(rename = "numberNotIns")]
    pub r#number_not_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterNumberNotIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringBeginsWiths")]
    pub r#string_begins_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringContains")]
    pub r#string_contains: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringEndsWiths")]
    pub r#string_ends_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringIns")]
    pub r#string_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringIn>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotBeginsWiths")]
    pub r#string_not_begins_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotContains")]
    pub r#string_not_contains: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotContain>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotEndsWiths")]
    pub r#string_not_ends_withs: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith>>,
    /// Compares a value of an event using multiple string values.
    #[builder(into)]
    #[serde(rename = "stringNotIns")]
    pub r#string_not_ins: Option<Vec<super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilterStringNotIn>>,
}
