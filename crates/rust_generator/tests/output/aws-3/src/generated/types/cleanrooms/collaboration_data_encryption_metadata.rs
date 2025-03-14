#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CollaborationDataEncryptionMetadata {
    #[builder(into)]
    #[serde(rename = "allowClearText")]
    pub r#allow_clear_text: Box<bool>,
    #[builder(into)]
    #[serde(rename = "allowDuplicates")]
    pub r#allow_duplicates: Box<bool>,
    #[builder(into)]
    #[serde(rename = "allowJoinsOnColumnsWithDifferentNames")]
    pub r#allow_joins_on_columns_with_different_names: Box<bool>,
    #[builder(into)]
    #[serde(rename = "preserveNulls")]
    pub r#preserve_nulls: Box<bool>,
}
