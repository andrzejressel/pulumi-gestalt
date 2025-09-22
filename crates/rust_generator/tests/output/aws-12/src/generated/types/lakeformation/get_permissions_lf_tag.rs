#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPermissionsLfTag {
    /// Identifier for the Data Catalog. By default, it is the account ID of the caller.
    #[builder(into)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: String,
    /// Key-name for the tag.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// List of possible values an attribute can take.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
