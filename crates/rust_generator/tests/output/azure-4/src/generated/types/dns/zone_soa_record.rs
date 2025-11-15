#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZoneSoaRecord {
    /// The email contact for the SOA record.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// The expire time for the SOA record. Defaults to `2419200`.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Option<i32>,
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// The domain name of the authoritative name server for the SOA record. If not set, computed value from Azure will be used.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Option<i32>,
    /// The refresh time for the SOA record. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "refreshTime")]
    pub r#refresh_time: Option<i32>,
    /// The retry time for the SOA record. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "retryTime")]
    pub r#retry_time: Option<i32>,
    /// The serial number for the SOA record. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<i32>,
    /// A mapping of tags to assign to the Record Set.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The Time To Live of the SOA Record in seconds. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<i32>,
}
