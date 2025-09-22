#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetIndexDocumentMetadataConfigurationUpdate {
    /// Name of the index field. Minimum length of 1. Maximum length of 30.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Block that provides manual tuning parameters to determine how the field affects the search results. Documented below.
    #[builder(into)]
    #[serde(rename = "relevances")]
    pub r#relevances: Vec<super::super::types::kendra::GetIndexDocumentMetadataConfigurationUpdateRelevance>,
    /// Block that provides information about how the field is used during a search. Documented below.
    #[builder(into)]
    #[serde(rename = "searches")]
    pub r#searches: Vec<super::super::types::kendra::GetIndexDocumentMetadataConfigurationUpdateSearch>,
    /// Data type of the index field. Valid values are `STRING_VALUE`, `STRING_LIST_VALUE`, `LONG_VALUE`, `DATE_VALUE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
