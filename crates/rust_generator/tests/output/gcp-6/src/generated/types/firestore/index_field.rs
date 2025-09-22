#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IndexField {
    /// Indicates that this field supports operations on arrayValues. Only one of `order`, `arrayConfig`, and
    /// `vectorConfig` can be specified.
    /// Possible values are: `CONTAINS`.
    #[builder(into)]
    #[serde(rename = "arrayConfig")]
    pub r#array_config: Option<String>,
    /// Name of the field.
    #[builder(into)]
    #[serde(rename = "fieldPath")]
    pub r#field_path: Option<String>,
    /// Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.
    /// Only one of `order`, `arrayConfig`, and `vectorConfig` can be specified.
    /// Possible values are: `ASCENDING`, `DESCENDING`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// Indicates that this field supports vector search operations. Only one of `order`, `arrayConfig`, and
    /// `vectorConfig` can be specified. Vector Fields should come after the field path `__name__`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vectorConfig")]
    pub r#vector_config: Option<Box<super::super::types::firestore::IndexFieldVectorConfig>>,
}
