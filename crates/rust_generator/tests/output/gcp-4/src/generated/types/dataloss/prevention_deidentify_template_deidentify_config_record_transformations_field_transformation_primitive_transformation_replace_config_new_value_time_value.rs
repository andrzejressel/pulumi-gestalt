#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationPrimitiveTransformationReplaceConfigNewValueTimeValue {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[builder(into)]
    #[serde(rename = "nanos")]
    pub r#nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    #[builder(into)]
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i32>,
}
