#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCustomDocumentEnrichmentConfigurationInlineConfigurationConditionConditionOnValue {
    /// A date expressed as an ISO 8601 string. It is important for the time zone to be included in the ISO 8601 date-time format. As of this writing only UTC is supported. For example, `2012-03-25T12:30:10+00:00`.
    #[builder(into)]
    #[serde(rename = "dateValue")]
    pub r#date_value: Option<String>,
    /// A long integer value.
    #[builder(into)]
    #[serde(rename = "longValue")]
    pub r#long_value: Option<i32>,
    /// A list of strings.
    #[builder(into)]
    #[serde(rename = "stringListValues")]
    pub r#string_list_values: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}
