#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MultiRegionAccessPointDetailsPublicAccessBlock {
    #[builder(into, default)]
    #[serde(rename = "blockPublicAcls")]
    pub r#block_public_acls: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "blockPublicPolicy")]
    pub r#block_public_policy: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "ignorePublicAcls")]
    pub r#ignore_public_acls: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "restrictPublicBuckets")]
    pub r#restrict_public_buckets: Box<Option<bool>>,
}
