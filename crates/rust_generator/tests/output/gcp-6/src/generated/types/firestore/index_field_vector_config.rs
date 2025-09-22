#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IndexFieldVectorConfig {
    /// The resulting index will only include vectors of this dimension, and can be used for vector search
    /// with the same dimension.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Option<i32>,
    /// Indicates the vector index is a flat index.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "flat")]
    pub r#flat: Option<Box<super::super::types::firestore::IndexFieldVectorConfigFlat>>,
}
