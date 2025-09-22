#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CrossAccountAttachmentResource {
    /// IP address range, in CIDR format, that is specified as resource.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Option<String>,
    /// The endpoint ID for the endpoint that is specified as a AWS resource.
    #[builder(into)]
    #[serde(rename = "endpointId")]
    pub r#endpoint_id: Option<String>,
    /// The AWS Region where a shared endpoint resource is located.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}
