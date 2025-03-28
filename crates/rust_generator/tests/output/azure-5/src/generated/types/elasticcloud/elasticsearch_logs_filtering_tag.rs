#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ElasticsearchLogsFilteringTag {
    /// Specifies the type of action which should be taken when the Tag matches the `name` and `value`. Possible values are `Exclude` and `Include`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Specifies the name (key) of the Tag which should be filtered.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the value of the Tag which should be filtered.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
