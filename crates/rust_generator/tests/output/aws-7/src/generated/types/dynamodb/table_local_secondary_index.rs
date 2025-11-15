#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableLocalSecondaryIndex {
    /// Name of the index
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Only required with `INCLUDE` as a projection type; a list of attributes to project into the index. These do not need to be defined as attributes on the table.
    #[builder(into)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Option<Vec<String>>,
    /// One of `ALL`, `INCLUDE` or `KEYS_ONLY` where `ALL` projects every attribute into the index, `KEYS_ONLY` projects  into the index only the table and index hash_key and sort_key attributes ,  `INCLUDE` projects into the index all of the attributes that are defined in `non_key_attributes` in addition to the attributes that that`KEYS_ONLY` project.
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: String,
    /// Name of the range key.
    #[builder(into)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: String,
}
