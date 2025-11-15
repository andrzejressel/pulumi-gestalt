#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsqlSettingsIpConfigAuthorizedNetwork {
    /// The time when this access control entry expires in RFC 3339 format.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Option<String>,
    /// A label to identify this entry.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Input only. The time-to-leave of this access control entry.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<String>,
    /// The allowlisted value for the access control list.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
