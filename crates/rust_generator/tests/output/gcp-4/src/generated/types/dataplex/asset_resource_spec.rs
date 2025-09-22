#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetResourceSpec {
    /// Immutable. Relative name of the cloud resource that contains the data that is being managed within a lake. For example: `projects/{project_number}/buckets/{bucket_id}` `projects/{project_number}/datasets/{dataset_id}`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Optional. Determines how read permissions are handled for each asset and their associated tables. Only available to storage buckets assets. Possible values: DIRECT, MANAGED
    #[builder(into)]
    #[serde(rename = "readAccessMode")]
    pub r#read_access_mode: Option<String>,
    /// Required. Immutable. Type of resource. Possible values: STORAGE_BUCKET, BIGQUERY_DATASET
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
