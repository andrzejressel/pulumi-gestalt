#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAccessKeysAccessKey {
    /// Access key ID.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: String,
    /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the access key was created.
    #[builder(into)]
    #[serde(rename = "createDate")]
    pub r#create_date: String,
    /// Access key status. Possible values are `Active` and `Inactive`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
