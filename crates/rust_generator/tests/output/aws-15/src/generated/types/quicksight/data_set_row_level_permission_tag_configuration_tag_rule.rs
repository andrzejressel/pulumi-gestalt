#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetRowLevelPermissionTagConfigurationTagRule {
    /// Column name that a tag key is assigned to.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: String,
    /// A string that you want to use to filter by all the values in a column in the dataset and don’t want to list the values one by one.
    #[builder(into)]
    #[serde(rename = "matchAllValue")]
    pub r#match_all_value: Option<String>,
    /// Unique key for a tag.
    #[builder(into)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: String,
    /// A string that you want to use to delimit the values when you pass the values at run time.
    #[builder(into)]
    #[serde(rename = "tagMultiValueDelimiter")]
    pub r#tag_multi_value_delimiter: Option<String>,
}
