#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpcEndpointServicePrivateDnsNameConfiguration {
    /// Name of the record subdomain the service provider needs to create.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Verification state of the VPC endpoint service. Consumers of the endpoint service can use the private name only when the state is `verified`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Endpoint service verification type, for example `TXT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Value the service provider adds to the private DNS name domain record before verification.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
