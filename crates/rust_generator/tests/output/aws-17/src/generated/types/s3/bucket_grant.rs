#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketGrant {
    /// Canonical user id to grant for. Used only when `type` is `CanonicalUser`.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// List of permissions to apply for grantee. Valid values are `READ`, `WRITE`, `READ_ACP`, `WRITE_ACP`, `FULL_CONTROL`.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Vec<String>,
    /// Type of grantee to apply for. Valid values are `CanonicalUser` and `Group`. `AmazonCustomerByEmail` is not supported.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Uri address to grant for. Used only when `type` is `Group`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}
