#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatascanDataQualitySpecRuleStatisticRangeExpectation {
    /// The maximum column statistic value allowed for a row to pass this validation.
    /// At least one of minValue and maxValue need to be provided.
    #[builder(into)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Option<String>,
    /// The minimum column statistic value allowed for a row to pass this validation.
    /// At least one of minValue and maxValue need to be provided.
    #[builder(into)]
    #[serde(rename = "minValue")]
    pub r#min_value: Option<String>,
    /// column statistics.
    /// Possible values are: `STATISTIC_UNDEFINED`, `MEAN`, `MIN`, `MAX`.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: String,
    /// Whether column statistic needs to be strictly lesser than ('<') the maximum, or if equality is allowed.
    /// Only relevant if a maxValue has been defined. Default = false.
    #[builder(into)]
    #[serde(rename = "strictMaxEnabled")]
    pub r#strict_max_enabled: Option<bool>,
    /// Whether column statistic needs to be strictly greater than ('>') the minimum, or if equality is allowed.
    /// Only relevant if a minValue has been defined. Default = false.
    #[builder(into)]
    #[serde(rename = "strictMinEnabled")]
    pub r#strict_min_enabled: Option<bool>,
}
