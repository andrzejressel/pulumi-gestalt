#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketMin {
    /// Represents a whole or partial calendar date.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketMinDateValue>>,
    /// Represents a day of the week.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeekValue")]
    pub r#day_of_week_value: Option<String>,
    /// A float value.
    #[builder(into)]
    #[serde(rename = "floatValue")]
    pub r#float_value: Option<f64>,
    /// An integer value (int64 format)
    #[builder(into)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Option<String>,
    /// A string value.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
    /// Represents a time of day.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeValue")]
    pub r#time_value: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformationsFieldTransformationInfoTypeTransformationsTransformationPrimitiveTransformationBucketingConfigBucketMinTimeValue>>,
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Option<String>,
}
