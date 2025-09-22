#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubSharedAccessPolicy {
    /// The name of the shared access policy.
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Option<String>,
    /// The permissions assigned to the shared access policy.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Option<String>,
    /// The primary key.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Option<String>,
    /// The secondary key.
    #[builder(into)]
    #[serde(rename = "secondaryKey")]
    pub r#secondary_key: Option<String>,
}
