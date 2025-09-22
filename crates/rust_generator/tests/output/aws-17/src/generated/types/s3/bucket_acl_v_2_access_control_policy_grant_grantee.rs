#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketAclV2AccessControlPolicyGrantGrantee {
    /// Display name of the owner.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Email address of the grantee. See [Regions and Endpoints](https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region) for supported AWS regions where this argument can be specified.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: Option<String>,
    /// Canonical user ID of the grantee.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Type of grantee. Valid values: `CanonicalUser`, `AmazonCustomerByEmail`, `Group`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// URI of the grantee group.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}
