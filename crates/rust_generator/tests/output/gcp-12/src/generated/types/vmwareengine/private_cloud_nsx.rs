#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PrivateCloudNsx {
    /// Fully qualified domain name of the appliance.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// Internal IP address of the appliance.
    #[builder(into)]
    #[serde(rename = "internalIp")]
    pub r#internal_ip: Option<String>,
    /// State of the appliance.
    /// Possible values are: `ACTIVE`, `CREATING`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Version of the appliance.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
